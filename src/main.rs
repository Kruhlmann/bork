use crossterm::event::{Event, KeyCode};
use io::input_handler::InputHandler;
use render::screen::Screen;
use ui::dialog::Dialog;
use world::{level::Level, tile::Tile};

mod io;
mod render;
mod schematic;
mod ui;
mod world;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut screen = Screen::new();
    let tiles: Vec<Tile> = (0..20_000)
        .map(|i| {
            let character = ((i % 26) as u8 + b'a') as char;
            Tile::new(character, None)
        })
    .collect();
    let level = Arc::new(Mutex::new(Level::new(
        tiles,
        200,
        100,
    )));

    let running = Arc::new(Mutex::new(true));
    let running_clone = Arc::clone(&running);
    let showdia = Arc::new(Mutex::new(false));
    let showdia_clone = Arc::clone(&showdia);
    let x_offset = Arc::new(Mutex::new(0));
    let x_offset_clone = Arc::clone(&x_offset);
    let y_offset = Arc::new(Mutex::new(0));
    let y_offset_clone = Arc::clone(&y_offset);
    let target_fps = 60;
    let sleep_duration = Duration::from_secs(1) / target_fps;

    let render_thread = thread::spawn(move || {
        let mut running_lock = running_clone.lock().unwrap();
        while *running_lock {
            drop(running_lock);

            let now = Instant::now();
            let render_level = level.lock().unwrap().clone();
            let x_offset_lock = x_offset_clone.lock().unwrap();
            let y_offset_lock = y_offset_clone.lock().unwrap();

            screen.render(&render_level, *x_offset_lock, *y_offset_lock);


            let showdia_lock = showdia_clone.lock().unwrap();
            if *showdia_lock {
                drop(showdia_lock);

                screen.hide_dialog();
                screen.show_dialog(Dialog::new(format!("Hi, there yea {x_offset_lock}:{y_offset_lock}")));
            }
            drop(x_offset_lock);
            drop(y_offset_lock);

            let elapsed = now.elapsed();
            if elapsed < sleep_duration {
                thread::sleep(sleep_duration - elapsed);
            }

            running_lock = running_clone.lock().unwrap();
        }
    });

    let update_thread = thread::spawn(move || {
        let input_handler = InputHandler::new();
        while *running.lock().unwrap() {
            let mut x_offset_lock = x_offset.lock().unwrap();
            let mut y_offset_lock = y_offset.lock().unwrap();
            if let Some(event) = input_handler.poll() {
                match event {
                    Event::Key(key_event) if key_event.code == KeyCode::Esc => {
                        let mut running_lock = running.lock().unwrap();
                        *running_lock = false;
                        drop(running_lock);
                    }
                    Event::Key(key_event) if key_event.code == KeyCode::Enter => {
                        let mut showdia_lock = showdia.lock().unwrap();
                        *showdia_lock = !*showdia_lock;
                        drop(showdia_lock);
                    }
                    Event::Key(key_event) if key_event.code == KeyCode::Char('w') => {
                        *y_offset_lock -= 1;
                    }
                    Event::Key(key_event) if key_event.code == KeyCode::Char('s') => {
                        *y_offset_lock += 1;
                    }
                    Event::Key(key_event) if key_event.code == KeyCode::Char('a') => {
                        *x_offset_lock -= 1;
                    }
                    Event::Key(key_event) if key_event.code == KeyCode::Char('d') => {
                        *x_offset_lock += 1;
                    }
                    _ => {}
                }
            }
            drop(x_offset_lock);
            drop(y_offset_lock);
        }
    });

    match render_thread.join() {
        Ok(_) => println!("Render thread finished successfully."),
        Err(err) => println!("Render thread panicked with error: {:?}", err),
    }
    match update_thread.join() {
        Ok(_) => println!("Update thread finished successfully."),
        Err(err) => println!("Update thread panicked with error: {:?}", err),
    }
}
