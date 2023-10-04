use crate::prelude::*;
use crate::systems::movement::randomized_move_delta;

#[system]
#[read_component(Point)]
#[read_component(MoveTowardsPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn move_towards_player(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
) {
    let mut rng = RandomNumberGenerator::new();
    let mut movers = <(Entity, &Point, &MoveTowardsPlayer, &FieldOfView)>::query();
    let mut player = <(Entity, &Point, &Player)>::query();

    let (player_entity, player_position, _) = player.iter(ecs).next().unwrap();

    let search_targets = vec![position_index(*player_position)];

    // Create a new Dijkstra map to calculate the distance from each enemy to the player
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers
        .iter(ecs)
        .for_each(|(entity, point, _, monster_fov)| {
            let mover_position_index = position_index(*point);
            let mut should_attack_player = false;
            let mut destination = Point::zero();

            if !monster_fov.visible_tiles.contains(player_position) {
                // If player is not visible, move randomly
                destination = *point + randomized_move_delta(&mut rng);
            } else if let Some(lowest_exist) =
                DijkstraMap::find_lowest_exit(&dijkstra_map, mover_position_index, map)
            {
                let distance = DistanceAlg::Pythagoras.distance2d(*point, *player_position);
                if distance > 1.2 {
                    destination = map.index_to_point2d(lowest_exist); // Move towards using Dijkstra map result
                } else {
                    should_attack_player = true; // Marks mover to attack player
                }
            }

            if should_attack_player {
                commands.push((
                    (),
                    AttackIntent {
                        attacker: *entity,
                        target: *player_entity,
                    },
                ));
            } else {
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
