#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod screens;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const HUD_WIDTH: i32 = SCREEN_WIDTH * 2;
    pub const HUD_HEIGHT: i32 = SCREEN_HEIGHT * 2;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::screens::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Systems,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rand = RandomNumberGenerator::new();

        let map_builder = MapBuilder::new(&mut rand);
        let start_point = map_builder.get_starting_point();

        spawn_player(&mut ecs, start_point);
        spawn_amulet_of_yala(&mut ecs, map_builder.get_amulet_point());

        // Spawn monsters over each room except the room player starts in.
        map_builder.rooms.iter().skip(1).for_each(|room| {
            spawn_monster(&mut ecs, &mut rand, room.center());
        });

        resources.insert(map_builder.map);
        resources.insert(Camera::new(start_point));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            systems: Systems::new(),
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
        *self = Self::new();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.clear_console(ctx);

        self.resources.insert(ctx.key); // Tick level resource

        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos())); // Mouse position resolved with map layer console

        self.systems
            .execute_turn(&mut self.ecs, &mut self.resources, ctx); // Executes turn based system

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            // Game Reset key
            let turn_state =* self.resources.get::<TurnState>().unwrap();

            let allowed_states = vec![TurnState::GameOver, TurnState::GameWon];

            if allowed_states.contains(&turn_state) {
                self.reset();
            }
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

    main_loop(context, State::new())
}
