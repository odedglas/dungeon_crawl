use crate::prelude::*;

#[system]
#[read_component(Item)]
#[read_component(AmuletOfYala)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(CarriedItem)]
#[read_component(Weapon)]
pub fn item_collector(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut TurnState,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    let (player_entity, player_position) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("Unable to find player position");

    let items: Vec<(&Entity, &Item, &Point)> = <(Entity, &Item, &Point)>::query()
        .iter(ecs)
        .map(|(entity, item, position)| (entity, item, position))
        .collect();

    for (entity, _, position) in &items {
        if **position == *player_position {
            let is_amulet = ecs
                .entry_ref(**entity)
                .unwrap()
                .get_component::<AmuletOfYala>()
                .is_ok();

            if is_amulet {
                turn_state.game_won();
                return;
            }

            if should_pick_item(key) {
                remove_existing_weapon(ecs, commands, &player_entity.clone(), entity);

                let der_entity = **entity;
                commands.remove_component::<Point>(der_entity); // Removes item from map
                commands.add_component(der_entity, CarriedItem(*player_entity)) // Carries item
            }
        }
    }
}

fn remove_existing_weapon(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    player_entity: &Entity,
    entity: &Entity,
) {
    if let Ok(e) = ecs.entry_ref(*entity) {
        if e.get_component::<Weapon>().is_ok() {
            // Remove existing weapon if present.
            <(Entity, &CarriedItem, &Weapon)>::query()
                .iter(ecs)
                .filter(|(_, carried_item, _)| carried_item.0 == *player_entity)
                .for_each(|(existing_weapon_entity, _, _)| {
                    commands.remove(*existing_weapon_entity);
                })
        }
    }
}

fn should_pick_item(key: &Option<VirtualKeyCode>) -> bool {
    if let Some(key) = key {
        if *key == VirtualKeyCode::G {
            return true;
        }
    }

    false
}
