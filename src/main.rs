#![warn(clippy::pedantic)]

mod camera;
mod components;
mod entities;
mod map;
mod map_builders;
mod screens;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::world::World;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const HUD_WIDTH: i32 = SCREEN_WIDTH * 2;
    pub const HUD_HEIGHT: i32 = SCREEN_HEIGHT * 2;
    pub const MAP_EDGE_PERIMETER: f32 = 2000.0;
    pub const MAX_LEVEL: usize = 3;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::entities::*;
    pub use crate::map::*;
    pub use crate::map_builders::*;
    pub use crate::screens::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::Level;
}

use legion::world::Duplicate;
use prelude::*;
use std::collections::HashSet;

pub type Level = usize;

struct State {
    ecs: World,
    resources: Resources,
    systems: Systems,
    level: Level,
    starting_point: Point,
}

fn create_world_merger() -> Duplicate {
    let mut duplicate = Duplicate::default();

    duplicate.register_clone::<Player>();
    duplicate.register_clone::<FieldOfView>();
    duplicate.register_clone::<Item>();
    duplicate.register_clone::<CarriedItem>();
    duplicate.register_clone::<Health>();
    duplicate.register_clone::<EntityName>();
    duplicate.register_clone::<Render>();
    duplicate.register_clone::<Point>();
    duplicate.register_clone::<MapRevealer>();
    duplicate.register_clone::<HealingPotion>();

    duplicate
}

impl State {
    pub fn new(level: Option<usize>, ecs: Option<World>) -> Self {
        let mut ecs = ecs.unwrap_or_default();
        let mut resources = Resources::default();
        let mut rand = RandomNumberGenerator::new();
        let level = level.unwrap_or(1);

        let architect = create_map_architect(&mut rand, level);
        let start_point = architect.get_starting_point();

        if level == 1 {
            // Player is being spawned only on the first level and carried towards next one.
            spawn_player(&mut ecs, start_point);
        }

        // Spawn monsters over each room except the room player starts in.
        let theme = architect.get_map_theme(&mut rand);
        let map_builder = architect.get_map_builder();

        map_builder
            .spawned_monsters
            .iter()
            .for_each(|monster_position| {
                spawn_monster(&mut ecs, &mut rand, *monster_position);
            });

        architect
            .get_map_items(&mut rand, level)
            .iter()
            .filter(|(map_item, _)| map_item != &GameEntity::AmuletOfYala || level == MAX_LEVEL)
            .for_each(|(map_item, item_position)| {
                spawn_map_item(&mut ecs, map_item, *item_position);
            });

        // Shared global game Resources
        resources.insert(map_builder.map.clone());
        resources.insert(theme);
        resources.insert(Camera::new(start_point));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            systems: Systems::new(),
            level,
            starting_point: start_point,
        }
    }

    pub fn clear_console(&mut self, ctx: &mut BTerm) {
        let console_layers = 2;
        for console_index in 0..=console_layers {
            ctx.set_active_console(console_index);
            ctx.cls();
        }
    }

    pub fn reset(&mut self) {
        let current_level = Some(self.level);
        *self = Self::new(current_level, None);
    }

    pub fn next_level(&mut self) {
        let mut new_world = World::default();

        let next_level = Some(self.level + 1);

        let mut duplicate = create_world_merger();
        let mut entities_to_keep: HashSet<Entity> = HashSet::new(); // Group entities to keep from this world

        let player_entity = <Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .find_map(|entity| Some(*entity))
            .unwrap();

        entities_to_keep.insert(player_entity);

        <(Entity, &CarriedItem)>::query()
            .iter(&self.ecs)
            .map(|(entity, _)| *entity)
            .for_each(|entity| {
                entities_to_keep.insert(entity);
            });

        for entity in &entities_to_keep {
            new_world.clone_from_single(&self.ecs, *entity, &mut duplicate);
        }

        *self = Self::new(next_level, Some(new_world));

        let map_start = self.starting_point; // Adjusting player into new world dimensions.

        <(Entity, &mut Point, &mut FieldOfView)>::query()
            .filter(component::<Player>())
            .iter_mut(&mut self.ecs)
            .for_each(|(_entity, position, fov)| {
                fov.mark_dirty(); // Marking its FOV as dirty so it will be re-calculated
                *position = map_start;
            });
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.clear_console(ctx);

        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos())); // Mouse position resolved with map layer console
        self.resources.insert(self.level); // Game level

        let turn_state = *self.resources.get::<TurnState>().unwrap();
        if turn_state == TurnState::AwaitingInput {
            // Only update pressed key when awaiting input
            self.resources.insert(ctx.key);
        }

        self.systems
            .execute_turn(&mut self.ecs, &mut self.resources, ctx); // Executes turn based system

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            // Game Reset key
            if [TurnState::GameOver, TurnState::GameWon].contains(&turn_state) {
                self.reset();
            }
        }

        if turn_state == TurnState::NextLevel {
            self.next_level();
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // Main font
        .with_font("terminal8x8.png", 8, 8) // HUD font
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Map layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Entity layer
        .with_simple_console_no_bg(HUD_WIDTH, HUD_HEIGHT, "terminal8x8.png") // HUD layer
        .build()?;

    main_loop(context, State::new(None, None))
}
