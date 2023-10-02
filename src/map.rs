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

pub fn position_index(point: Point) -> usize {
    ((point.y * SCREEN_WIDTH) + point.x) as usize
}

pub fn safe_position_index(point: Point) -> Option<usize> {
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

    pub fn in_screen_bounds(point: Point) -> bool {
        point.y >= 0 && point.y < SCREEN_HEIGHT && point.x >= 0 && point.x < SCREEN_WIDTH
    }

    pub fn can_enter_cell(&self, position: Point) -> bool {
        Self::in_screen_bounds(position) && self.cells[position_index(position)] == CellType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        Self::in_screen_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(index);

        let possible_directions = vec![
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ];

        for possible_point in &possible_directions {
            let destination = *possible_point + location;

            if self.can_enter_cell(destination) {
                let destination_index = self.point2d_to_index(destination);
                exits.push((destination_index, 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        let point1 = self.index_to_point2d(index1);
        let point2 = self.index_to_point2d(index2);

        DistanceAlg::Pythagoras.distance2d(point1, point2)
    }
}
