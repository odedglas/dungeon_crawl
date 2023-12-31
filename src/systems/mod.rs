mod combat;
mod entity_render;
mod fields_of_view;
mod hud;
mod item_collector;
mod map_render;
mod move_towards_player;
mod movement;
mod player_input;
mod tooltip;
mod turn_transition;
mod use_items;

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

    pub fn execute_turn(&mut self, ecs: &mut World, resources: &mut Resources, ctx: &mut BTerm) {
        let current_state = *resources.get::<TurnState>().unwrap();

        match current_state {
            TurnState::AwaitingInput => self.await_input_system.execute(ecs, resources),
            TurnState::PlayerTurn => {
                self.player_system.execute(ecs, resources);
            }
            TurnState::MonsterTurn => self.monster_system.execute(ecs, resources),
            TurnState::GameOver => game_over_display(ctx),
            TurnState::GameWon => game_won_display(ctx),
            TurnState::NextLevel => {}
        }
    }
}

fn build_await_user_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fields_of_view::fields_of_view_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltip::tooltip_system())
        .build()
}

fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_items::use_items_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fields_of_view::fields_of_view_system())
        .flush()
        .add_system(item_collector::item_collector_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(turn_transition::turn_transition_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(move_towards_player::move_towards_player_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fields_of_view::fields_of_view_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(turn_transition::turn_transition_system())
        .build()
}
