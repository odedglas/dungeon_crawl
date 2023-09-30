use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(RandomMovement)]
pub fn random_movement(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &mut Point, &RandomMovement)>::query();

    movers.iter_mut(ecs).for_each(|(entity, point, _)| {
        commands.push((
            (),
            MoveIntent {
                entity: *entity,
                current: *point,
                movement_type: Movement::Random,
            },
        ));
    });
}
