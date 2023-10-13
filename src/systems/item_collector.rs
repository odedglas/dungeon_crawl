use crate::prelude::*;

#[system]
#[read_component(Item)]
#[read_component(AmuletOfYala)]
#[read_component(Player)]
#[read_component(Point)]
pub fn item_collector(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_state: &mut TurnState,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    let player_position = <&Point>::query()
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
                commands.remove_component::<Point>(**entity); // Removes item from map
                commands.add_component(**entity, CarriedItem) // Carries item
            }
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
