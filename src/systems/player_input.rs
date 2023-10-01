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
    #[resource] turn_state: &mut TurnState,
) {
    let delta = get_keyboard_delta(key);
    if let Some(delta) = delta {
        let mut players = <(Entity, &Point, &mut Health)>::query().filter(component::<Player>());

        let (player_entity, destination, health) = players
            .iter_mut(ecs)
            .find_map(|(entity, pos, health)| Some((*entity, *pos + delta, health)))
            .unwrap();

        if delta == Point::zero() {
            health.current = i32::min(health.max, health.current + 1);
            turn_state.next();
            return;
        }

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

        turn_state.next();
    }
}
