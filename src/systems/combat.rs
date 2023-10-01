use crate::prelude::*;

#[system]
#[read_component(AttackIntent)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &AttackIntent)>::query();

    let targets: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    for (message, target) in &targets {
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("[Combat System] Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*target);
            }
            println!("[Combat System] Health after attack: {}", health.current);
        }
        commands.remove(*message);
    }
}
