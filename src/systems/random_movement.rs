use crate::prelude::*;

fn randomized_move_delta(rand: &mut RandomNumberGenerator) -> Point {
    match rand.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    }
}

#[system]
#[write_component(Point)]
#[read_component(RandomMovement)]
pub fn random_movement(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &RandomMovement)>::query();

    movers.iter_mut(ecs).for_each(|(point, _)| {
        let mut rand = RandomNumberGenerator::new();
        let destination = *point + randomized_move_delta(&mut rand);

        if map.can_enter_cell(destination) {
            *point = destination;
        }
    })
}
