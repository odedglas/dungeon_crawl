use super::MapArchitect;
use crate::prelude::*;
use derives::BaseMapArchitect;

#[derive(BaseMapArchitect)]
pub struct CellularAutomataArchitect {
    pub map_builder: MapBuilder,
}

impl CellularAutomataArchitect {
    pub fn new() -> Self {
        Self {
            map_builder: MapBuilder::empty(),
        }
    }

    fn random_noise_map(&mut self, rand: &mut RandomNumberGenerator) {
        self.map_builder.map.cells.iter_mut().for_each(|cell_type| {
            let roll = rand.range(0, 100);
            *cell_type = match roll {
                0..=55 => CellType::Floor,
                _ => CellType::Wall,
            };
        });
    }

    fn count_neighbors(point: Point, map: &Map) -> usize {
        let mut neighbors = 0;

        for y in -1..=1 {
            for x in -1..=1 {
                let is_own_point = x == 0 && y == 0;

                if !is_own_point
                    && map.cells[position_index(point + Point::new(x, y))] == CellType::Wall
                {
                    neighbors += 1; // Walls will be considered within this algorithm
                }
            }
        }

        neighbors
    }

    fn adjust_by_neighbors(&mut self) {
        let map = &self.map_builder.map;
        let mut new_cells = map.cells.clone();

        // Executes 10 iterations to adjust the map with neighbors count
        for _ in 0..10 {
            for y in 1..SCREEN_HEIGHT - 1 {
                // Iterates map, leaving edges aside.
                for x in 1..SCREEN_WIDTH - 1 {
                    let point = Point::new(x, y);

                    let neighbors = Self::count_neighbors(point, map);
                    let index = position_index(point);

                    if neighbors > 4 || neighbors == 0 {
                        new_cells[index] = CellType::Wall;
                    } else {
                        new_cells[index] = CellType::Floor;
                    }
                }
            }
        }

        self.map_builder.map.cells = new_cells;
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rand: &mut RandomNumberGenerator) {
        self.random_noise_map(rand);

        self.adjust_by_neighbors();
    }
}
