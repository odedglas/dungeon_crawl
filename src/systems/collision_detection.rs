use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Monster)]
pub fn collision_detection(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_position = get_player_position(ecs);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Monster>());

    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_position)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}

fn get_player_position(ecs: &SubWorld) -> Point {
    let mut player_pos = Point::zero();

    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    player_pos
}
