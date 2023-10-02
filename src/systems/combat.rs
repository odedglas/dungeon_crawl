use crate::prelude::*;

#[system]
#[read_component(AttackIntent)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut TurnState,
) {
    let mut attackers = <(Entity, &AttackIntent)>::query();

    let targets: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    for (message, target) in &targets {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("[Combat System] Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                if is_player {
                    turn_state.game_over();
                } else {
                    commands.remove(*target); // Removes entity from world in case it died.
                }
            }
            println!("[Combat System] Health after attack: {}", health.current);
        }
        commands.remove(*message);
    }
}
