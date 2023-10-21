use crate::prelude::*;

#[system]
#[read_component(AttackIntent)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(CarriedItem)]
#[read_component(Weapon)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut TurnState,
) {
    let mut attackers = <(Entity, &AttackIntent)>::query();

    let targets: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target, attack.attacker))
        .collect();

    for (message, target, attacker) in &targets {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let mut attacker_damage = ecs
            .entry_ref(*attacker)
            .unwrap()
            .get_component::<Damage>()
            .unwrap_or(&Damage(1))
            .0;

        let weapon_damage: i32 = <(&CarriedItem, &Weapon)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, weapon)| weapon.0)
            .sum();

        attacker_damage += weapon_damage;
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!(
                "[Combat System] Health before attack: {} / {}",
                health.current, attacker_damage
            );
            health.current -= attacker_damage;
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
