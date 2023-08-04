use std::{
    alloc::Layout,
    io::{self, Stdout},
};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::world::level::Level;

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Screen {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        crossterm::execute!(terminal.backend_mut(), EnterAlternateScreen).unwrap();
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        terminal.clear().unwrap();

        Self { terminal }
    }

    pub fn render_level(&mut self, level: &Level) {
        let tiles = level.get_tiles();
        let width = level.get_width() as usize;
        let height = level.get_height() as usize;

        self.terminal
            .draw(|f| {
                let terminal_size = f.size();
                let x_offset = (terminal_size.width as usize - width) / 2;
                let y_offset = (terminal_size.height as usize - height) / 2;

                for y in 0..height {
                    let start = y * width;
                    let end = start + width;
                    let row_tiles = &tiles[start..end];
                    let spans: Vec<_> = row_tiles
                        .iter()
                        .map(|tile| Span::styled(tile.symbol.to_string(), tile.style))
                        .collect();
                    let paragraph = Paragraph::new(Spans::from(spans));
                    f.render_widget(
                        paragraph,
                        Rect::new(x_offset as u16, (y + y_offset) as u16, width as u16, 1),
                    );
                }
            })
            .unwrap();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        crossterm::execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}
