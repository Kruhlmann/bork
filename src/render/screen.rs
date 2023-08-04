use std::io::{self, Stdout};

// use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use crate::{ui::dialog::Dialog, world::{level::Level, tile::Tile}};

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    dialog: Option<Dialog>,
}

pub struct BoxParagraph<'a> {
    pub paragraph: Paragraph<'a>,
    pub dimensions: Rect,
}

impl Screen {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        // crossterm::execute!(terminal.backend_mut(), EnterAlternateScreen).unwrap();
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        terminal.clear().unwrap();

        Self {
            terminal,
            dialog: None,
        }
    }

    fn create_dialog_box_paragraphs(
        terminal_size: Rect,
        dialog: Option<Dialog>,
    ) -> Vec<BoxParagraph<'static>> {
        match dialog {
            Some(dialog) => {
                let dialog_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(tui::widgets::BorderType::Double);

                let contents = dialog.contents.clone();
                let paragraph = Paragraph::new(Spans::from(contents))
                    .block(dialog_block)
                    .wrap(Wrap { trim: true });

                let dimensions = Rect::new(0, terminal_size.height - 4, terminal_size.width, 4);
                vec![BoxParagraph {
                    paragraph,
                    dimensions,
                }]
            }
            None => vec![],
        }
    }

    fn create_level_box_paragraphs(
        level: &Level,
        terminal_size: Rect,
        x_offset: usize,
        y_offset: usize,
    ) -> Vec<BoxParagraph> {
        let tiles = level.get_tiles();
        let width = level.get_width() as usize;
        let height = level.get_height() as usize;
        let mut visible_tiles: Vec<Tile> = Vec::new();
        for y in y_offset..terminal_size.height as usize + y_offset {
            for x in x_offset..terminal_size.width as usize + x_offset {
                visible_tiles.push(tiles[x + y * width].clone());
            }
        }
        let visible_spans: Vec<Span> = visible_tiles
            .iter()
            .map(|tile|Span::styled(tile.symbol.to_string(), tile.style))
            .collect();
        vec![BoxParagraph {
            paragraph: Paragraph::new(Spans::from(visible_spans)).wrap(Wrap { trim: true }),
            dimensions: Rect::new(0, 0, terminal_size.width, terminal_size.height),
        }]

    }

    pub fn render(&mut self, level: &Level, x_offset: usize, y_offset: usize) {
        let terminal_size = self.terminal.size().unwrap();
        let mut box_paragraphs = Vec::new();
        box_paragraphs.extend(Screen::create_level_box_paragraphs(
            level,
            terminal_size,
            x_offset,
            y_offset,
        ));
        box_paragraphs.extend(Screen::create_dialog_box_paragraphs(
            terminal_size,
            self.dialog.clone(),
        ));

        self.terminal
            .draw(|frame| {
                for box_paragraph in box_paragraphs {
                    frame.render_widget(box_paragraph.paragraph, box_paragraph.dimensions);
                }
            })
            .unwrap();
    }

    pub fn show_dialog(&mut self, dialog: Dialog) {
        self.dialog = Some(dialog);
    }

    pub fn hide_dialog(&mut self) {
        self.dialog = None;
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        // crossterm::execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}
