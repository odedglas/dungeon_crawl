use crate::prelude::*;

pub fn get_keyboard_delta(key: &Option<VirtualKeyCode>) -> Option<Point> {
    if let Some(key) = key {
        return Option::from(match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        });
    }

    None
}

pub fn randomized_move_delta(rand: &mut RandomNumberGenerator) -> Point {
    match rand.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    }
}

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    move_intent: &MoveIntent,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    commands: &mut CommandBuffer,
) {
    let destination = move_intent.to;

    if map.can_enter_cell(destination) {
        commands.add_component(move_intent.entity, destination); // Moves component to new position

        if move_intent.movement_type == Movement::Keyboard {
            camera.center(destination); // Center around player
        }
    }
    commands.remove(*entity); // Clear intent after it was processed
}
