use tui::style::{Color, Style};

#[derive(Clone)]
pub struct Tile {
    pub symbol: char,
    pub style: Style,
}

impl Tile {
    pub fn new(symbol: char, style: Option<Style>) -> Self {
        let style = style.unwrap_or_else(|| Style::default().fg(Color::White));
        Self { symbol, style }
    }
}
