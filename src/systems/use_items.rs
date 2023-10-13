use crate::prelude::*;

fn get_item_activation_key(key: &Option<VirtualKeyCode>) -> Option<i32> {
    if let Some(key) = key {
        return match key {
            VirtualKeyCode::Key1 => Some(0),
            VirtualKeyCode::Key2 => Some(1),
            VirtualKeyCode::Key3 => Some(2),
            VirtualKeyCode::Key4 => Some(3),
            VirtualKeyCode::Key5 => Some(4),
            VirtualKeyCode::Key6 => Some(5),
            VirtualKeyCode::Key7 => Some(6),
            VirtualKeyCode::Key8 => Some(7),
            VirtualKeyCode::Key9 => Some(8),
            _ => None,
        };
    }

    None
}

#[system]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(CarriedItem)]
#[read_component(HealingPotion)]
#[read_component(MapRevealer)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    if let Some(activation_index) = get_item_activation_key(key) {
        let mut healing_amount = 0;

        let item_entity = <(Entity, &Item, &CarriedItem)>::query() // Searching for item with activation index
            .iter(ecs)
            .enumerate()
            .filter(|(count, (_, _item, _))| *count == activation_index as usize)
            .map(|(_, (entity, _, _))| *entity)
            .next();

        if let Some(item_entity) = item_entity {
            // Handling Item's activation
            if let Ok(item) = ecs.entry_ref(item_entity) {
                if let Ok(healing) = item.get_component::<HealingPotion>() {
                    healing_amount = healing.heal_amount; // Overcome borrow checker, as we still got immutable reference to `ecs` at this point
                }

                if let Ok(_map_revealer) = item.get_component::<MapRevealer>() {
                    map.reveal_map();
                }
            }

            if healing_amount > 0 {
                let player_health = <&mut Health>::query()
                    .filter(component::<Player>())
                    .iter_mut(ecs)
                    .find_map(Some)
                    .unwrap();

                player_health.current =
                    i32::min(player_health.max, player_health.current + healing_amount);
            }

            commands.remove(item_entity); // Removes item from inventory
        }
    }
}
