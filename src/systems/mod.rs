mod combat;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_movement;
mod tooltip;
mod turn_transition;

use crate::prelude::*;

pub struct Systems {
    pub await_input_system: Schedule,
    pub player_system: Schedule,
    pub monster_system: Schedule,
}

impl Systems {
    pub fn new() -> Self {
        Self {
            await_input_system: build_await_user_input_scheduler(),
            player_system: build_player_scheduler(),
            monster_system: build_monster_scheduler(),
        }
    }

    pub fn execute_turn(&mut self, ecs: &mut World, resources: &mut Resources) {
        let current_state = *resources.get::<TurnState>().unwrap();

        match current_state {
            TurnState::AwaitingInput => self.await_input_system.execute(ecs, resources),
            TurnState::PlayerTurn => {
                self.player_system.execute(ecs, resources);
            }
            TurnState::MonsterTurn => self.monster_system.execute(ecs, resources),
        }
    }
}

fn build_await_user_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltip::tooltip_system())
        .build()
}

fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(turn_transition::turn_transition_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_movement::random_movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(turn_transition::turn_transition_system())
        .build()
}
