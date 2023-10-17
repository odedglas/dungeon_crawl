use crate::prelude::*;

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: CellType) -> FontCharType;
}

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: CellType) -> FontCharType {
        match tile_type {
            CellType::Floor => to_cp437('.'),
            CellType::Wall => to_cp437('#'),
            CellType::Staircase => to_cp437('>'),
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: CellType) -> FontCharType {
        match tile_type {
            CellType::Floor => to_cp437(';'),
            CellType::Wall => to_cp437('"'),
            CellType::Staircase => to_cp437('>'),
        }
    }
}
