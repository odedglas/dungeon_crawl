use crate::prelude::*;
use std::cmp::{max, min};

const MAX_ROOMS: usize = 10;
const UNREACHABLE: &f32 = &f32::MAX;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
}

fn create_room_rect(rand: &mut RandomNumberGenerator) -> Rect {
    Rect::with_size(
        rand.range(1, SCREEN_WIDTH - 10),
        rand.range(1, SCREEN_HEIGHT - 10),
        rand.range(2, 10),
        rand.range(2, 10),
    )
}

impl MapBuilder {
    pub fn new(rand: &mut RandomNumberGenerator) -> Self {
        let mut builder = Self {
            map: Map::new(),
            rooms: vec![],
        };

        builder.init_map(rand);

        builder
    }

    fn init_map(&mut self, rand: &mut RandomNumberGenerator) {
        self.fill(CellType::Wall); // Map starts filled with walls

        self.build_random_rooms(rand); // Rooms are created randomly

        self.connect_rooms(rand); // Connecting rooms
    }

    fn build_random_rooms(&mut self, rand: &mut RandomNumberGenerator) {
        while self.rooms.len() < MAX_ROOMS {
            let room = create_room_rect(rand);

            let mut overlap = false;
            for r in &self.rooms {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // Mark room points as floor
                room.for_each(|room_point| {
                    if Map::in_screen_bounds(room_point) {
                        let floor_index = position_index(room_point);
                        self.map.cells[floor_index] = CellType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    pub fn get_starting_point(&self) -> Point {
        let first_room = self.rooms[0];

        first_room.center()
    }

    pub fn get_amulet_point(&self) -> Point {
        // Create Dijsktra map to find the furthest point from the player
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.get_starting_point())],
            &self.map,
            1024.0,
        );

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dijkstra_distance)| *dijkstra_distance < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // Farthest point tho reachable
                .unwrap()
                .0,
        )
    }

    fn fill(&mut self, cell_type: CellType) {
        self.map.cells.iter_mut().for_each(|c| *c = cell_type);
    }

    fn connect_rooms(&mut self, rand: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // Apply different connect ordering to create variance.
            if rand.range(0, 2) == 1 {
                self.apply_tunnel(prev.x, new.x, Point::new(0, prev.y));
                self.apply_tunnel(prev.y, new.y, Point::new(new.x, 0));
            } else {
                self.apply_tunnel(prev.y, new.y, Point::new(prev.x, 0));
                self.apply_tunnel(prev.x, new.x, Point::new(0, new.y));
            }
        }
    }

    fn apply_tunnel(&mut self, t1: i32, t2: i32, anchor: Point) {
        let max = max(t1, t2);
        let min = min(t1, t2);

        for value in min..=max {
            let position_anchor = if anchor.x == 0 {
                Point::new(value, anchor.y)
            } else {
                Point::new(anchor.x, value)
            };

            if let Some(index) = safe_position_index(position_anchor) {
                self.map.cells[index] = CellType::Floor;
            }
        }
    }
}
