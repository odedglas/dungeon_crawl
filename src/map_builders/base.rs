use crate::map_builders::themes::{DungeonTheme, ForestTheme, MapTheme};
use crate::prelude::*;

const UNREACHABLE: &f32 = &f32::MAX;
const NUM_MONSTERS: usize = 50;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub spawned_monsters: Vec<Point>,
    pub items: Vec<Point>,
    pub theme: Option<Box<dyn MapTheme>>,
}

impl MapBuilder {
    pub fn empty() -> Self {
        Self {
            map: Map::new(),
            rooms: vec![],
            spawned_monsters: vec![],
            items: vec![],
            theme: None,
        }
    }

    pub fn fill(&mut self, cell_type: CellType) {
        self.map.cells.iter_mut().for_each(|t| *t = cell_type);
    }

    pub fn find_most_distant(&self, starting_point: Point) -> Point {
        // Create Dijsktra map to find the furthest point from the player
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(starting_point)],
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

    pub fn map_center() -> Point {
        Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
    }

    pub fn random_map_point(rand: &mut RandomNumberGenerator) -> Point {
        Point::new(rand.range(0, SCREEN_WIDTH), rand.range(0, SCREEN_HEIGHT))
    }

    pub fn random_item_placement(&self, rand: &mut RandomNumberGenerator) -> Point {
        let mut placement = None;

        while placement.is_none() {
            let location = Self::random_map_point(rand);

            if self.map.can_enter_cell(location) {
                placement = Some(location)
            }
        }

        placement.unwrap()
    }
}

pub trait BaseMapArchitect {
    fn get_map_builder(&self) -> &MapBuilder;

    fn get_mut_map_builder(&mut self) -> &mut MapBuilder;
}

pub trait MapArchitect: BaseMapArchitect {
    fn get_map_theme(&self, rng: &mut RandomNumberGenerator) -> Box<dyn MapTheme> {
        match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        }
    }

    fn set_monsters(&mut self, rng: &mut RandomNumberGenerator) {
        let monsters_positions = self.monsters_positions(rng);

        self.get_mut_map_builder().spawned_monsters = monsters_positions;
    }

    fn place_next_level_staircase(&mut self) {
        let starting_point = self.get_starting_point();
        let map_builder = self.get_mut_map_builder();

        let farthest_point = map_builder.find_most_distant(starting_point);

        let exit_index = map_builder.map.point2d_to_index(farthest_point);
        map_builder.map.cells[exit_index] = CellType::Staircase;
    }

    fn build(&mut self, rng: &mut RandomNumberGenerator);

    fn get_starting_point(&self) -> Point {
        // Returns the closes tile to the center of the map
        let map = &self.get_map_builder().map;
        let center = MapBuilder::map_center();

        let closest_point_index = map
            .cells
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == CellType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point_index)
    }

    fn get_amulet_point(&self) -> Point {
        let map_builder = self.get_map_builder();
        map_builder.find_most_distant(self.get_starting_point())
    }

    fn monsters_positions(&self, rand: &mut RandomNumberGenerator) -> Vec<Point> {
        let map_builder = self.get_map_builder();
        let map = &map_builder.map;

        let mut potential_cells: Vec<Point> = map
            .cells
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == CellType::Floor
                    && DistanceAlg::Pythagoras
                        .distance2d(self.get_starting_point(), map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| map.index_to_point2d(idx))
            .collect();

        let mut monsters_positions = Vec::new();

        // Choosing randomly from potential cells
        for _ in 0..NUM_MONSTERS {
            let random_target_index = rand.random_slice_index(&potential_cells).unwrap();

            monsters_positions.push(potential_cells[random_target_index]);
            potential_cells.remove(random_target_index);
        }

        monsters_positions
    }

    fn get_map_items(
        &self,
        rng: &mut RandomNumberGenerator,
        level: usize,
    ) -> Vec<(GameEntity, Point)> {
        let map_builder = self.get_map_builder();

        let mut items: Vec<(GameEntity, Point)> = vec![(
            GameEntity::MapRevealer,
            map_builder.random_item_placement(rng),
        )];

        match level {
            // Level specific items
            1 => {
                items.push((
                    GameEntity::RustySword,
                    map_builder.random_item_placement(rng),
                ));
                items.push((
                    GameEntity::ShinySword,
                    map_builder.random_item_placement(rng),
                ));
            }
            2 => items.push((
                GameEntity::ShinySword,
                map_builder.random_item_placement(rng),
            )),
            MAX_LEVEL => {
                items.push((
                    GameEntity::HugeSword,
                    map_builder.random_item_placement(rng),
                ));
                items.push((GameEntity::AmuletOfYala, self.get_amulet_point()));
            }
            _ => panic!("Unknown game level {level}"),
        };

        for _ in 0..15 {
            let position = map_builder.random_item_placement(rng);
            let healing_amount = rng.range(1, 5);

            items.push((GameEntity::HealingPotion(healing_amount), position))
        }

        items
    }
}
