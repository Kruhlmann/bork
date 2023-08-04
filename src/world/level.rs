use super::tile::Tile;

#[derive(Clone)]
pub struct Level {
    width: u16,
    height: u16,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new(tiles: Vec<Tile>, width: u16, height: u16) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}
