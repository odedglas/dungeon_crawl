use crate::prelude::*;

#[system]
#[read_component(Item)]
#[read_component(AmuletOfYala)]
#[read_component(Player)]
#[read_component(Point)]
pub fn item_collector(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
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
            }
        }
    }
}
