use crate::prelude::*;
use crate::systems::movement::get_keyboard_delta;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Monster)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] map: &Map,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = get_keyboard_delta(key);

        if delta == Point::zero() {
            turn_state.next();
            return;
        }

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let (player_entity, destination) = players
            .iter_mut(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        // Check if player destination collides with Monster entity
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Monster>());
        let mut attacking = false;

        enemies
            .iter_mut(ecs)
            .filter(|(_, pos)| **pos == destination)
            .for_each(|(entity, _)| {
                attacking = true;
                commands.push((
                    (),
                    AttackIntent {
                        attacker: player_entity,
                        target: *entity,
                    },
                ));
            });

        if !attacking {
            // Push move intent processed by movement system
            commands.push((
                (),
                MoveIntent {
                    entity: player_entity,
                    to: destination,
                    movement_type: Movement::Keyboard,
                },
            ));
        }

        let cell_type = map.cells[map.point2d_to_index(destination)];
        if cell_type == CellType::Staircase {
            turn_state.next_level();
        } else {
            turn_state.next();
        }
    }
}
