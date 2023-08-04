use crossterm::event::{KeyCode, Event};
use io::input_handler::InputHandler;
use render::screen::Screen;
use world::{level::Level, tile::Tile};

mod render;
mod world;
mod io;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut screen = Screen::new();
    let level = Arc::new(Mutex::new(Level::new(
        vec![Tile::new('.', None), Tile::new('.', None)],
        1,
        2,
    )));

    let running = Arc::new(Mutex::new(true));
    let running_clone = Arc::clone(&running);

    let target_fps = 60;
    let sleep_duration = Duration::from_secs(1) / target_fps;

    let render_thread = thread::spawn(move || {
        while *running_clone.lock().unwrap() {
            let now = Instant::now();
            let render_level = level.lock().unwrap().clone();

            screen.render_level(&render_level);

            let elapsed = now.elapsed();
            if elapsed < sleep_duration {
                thread::sleep(sleep_duration - elapsed);
            }
        }
    });

    let update_thread = thread::spawn(move || {
        let input_handler = InputHandler::new();
        while *running.lock().unwrap() {
            if let Some(event) = input_handler.poll() {
                match event {
                    Event::Key(key_event) if key_event.code == KeyCode::Esc => {
                        *running.lock().unwrap() = false;
                    }
                    _ => {}
                }
            }
        }
    });

    render_thread.join().unwrap();
    update_thread.join().unwrap();
}
