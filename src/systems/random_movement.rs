use crate::prelude::*;
use crate::systems::movement::randomized_move_delta;

#[system]
#[read_component(Point)]
#[read_component(RandomMovement)]
#[read_component(Player)]
pub fn random_movement(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();
    let mut movers = <(Entity, &Point, &RandomMovement)>::query();
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    movers.iter(ecs).for_each(|(entity, point, _)| {
        let destination = *point + randomized_move_delta(&mut rng);
        let mut attacking = false;

        players.iter(ecs).for_each(|(player_entity, player_pos)| {
            if *player_pos == destination {
                commands.push((
                    (),
                    AttackIntent {
                        attacker: *entity,
                        target: *player_entity,
                    },
                ));
                attacking = true;
            }
        });

        if !attacking {
            commands.push((
                (),
                MoveIntent {
                    entity: *entity,
                    to: destination,
                    movement_type: Movement::Random,
                },
            ));
        }
    });
}
