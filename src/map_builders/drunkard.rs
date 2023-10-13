use super::MapArchitect;
use crate::prelude::*;
use derives::BaseMapArchitect;

// The maximum number of steps the drunkard can take before stopping and considered as "dead".
const STAGGER_DISTANCE: usize = 400;
const MAP_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const FLOORS_EXPOSURE_RATE: usize = MAP_TILES / 3;

#[derive(BaseMapArchitect)]
pub struct DrunkardArchitect {
    pub map_builder: MapBuilder,
}

impl DrunkardArchitect {
    pub fn new() -> Self {
        Self {
            map_builder: MapBuilder::empty(),
        }
    }

    // Apply the drunkard's algorithm on a given starting point.
    fn drunkard_point(&mut self, start: &Point, rand: &mut RandomNumberGenerator) {
        let map = &mut self.map_builder.map;
        let mut drunkard_pos = *start;
        let mut distance_staggered = 0;

        loop {
            let drunk_index = map.point2d_to_index(drunkard_pos);
            map.cells[drunk_index] = CellType::Floor; // Mark current as floor

            match rand.range(0, 4) {
                // Randomly moves drunkard
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            if !map.in_bounds(drunkard_pos) {
                // If drunkard is out of bounds, stop
                break;
            }

            distance_staggered += 1;

            if distance_staggered > STAGGER_DISTANCE {
                // If drunkard has staggered too much, stop
                break;
            }
        }
    }

    fn has_enough_floor(&self) -> bool {
        self.map_builder
            .map
            .cells
            .iter()
            .filter(|cell| **cell == CellType::Floor)
            .count()
            > FLOORS_EXPOSURE_RATE
    }

    fn close_center_perimeter(&mut self) {
        let center = MapBuilder::map_center();
        let map = &mut self.map_builder.map;

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[map.point2d_to_index(center)],
            map,
            1024.0,
        );

        dijkstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, distance)| **distance > MAP_EDGE_PERIMETER)
            .for_each(|(idx, _)| map.cells[idx] = CellType::Wall);
    }
}

impl MapArchitect for DrunkardArchitect {
    fn build(&mut self, rand: &mut RandomNumberGenerator) {
        self.map_builder.fill(CellType::Wall); // Starts with a full wall map

        let screen_center = MapBuilder::map_center();
        self.drunkard_point(&screen_center, rand); // Starts the drunkard's algorithm on the center of the screen

        // Check if map has enough floor cells generated
        while !self.has_enough_floor() {
            let random_map_start = MapBuilder::random_map_point(rand);
            self.drunkard_point(&random_map_start, rand); // Starts the drunkard's algorithm on a random point

            self.close_center_perimeter(); // Since random point was chosen from one of map edges, we manually close edges of the map
        }
    }
}
