use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellType {
    Wall,
    Floor,
}

pub struct Map {
    pub cells: Vec<CellType>,
}

pub fn position_index(point: &Point) -> usize {
    ((point.y * SCREEN_WIDTH) + point.x) as usize
}

pub fn safe_position_index(point: &Point) -> Option<usize> {
    if !Map::in_screen_bounds(point) {
        return None;
    }

    Some(position_index(point))
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: vec![CellType::Wall; NUM_TILES],
        }
    }

    pub fn in_screen_bounds(point: &Point) -> bool {
        point.y >= 0 && point.y < SCREEN_HEIGHT && point.x >= 0 && point.x < SCREEN_WIDTH
    }

    pub fn can_enter_cell(&self, position: Point) -> bool {
        Self::in_screen_bounds(&position)
            && self.cells[position_index(&position)] == CellType::Floor
    }
}
