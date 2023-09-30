use crate::prelude::*;

fn get_keyboard_delta(key: VirtualKeyCode) -> Option<Point> {
    match key {
        VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::Down => Some(Point::new(0, 1)),
        _ => None,
    }
}

fn randomized_move_delta(rand: &mut RandomNumberGenerator) -> Option<Point> {
    Option::from(match rand.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    })
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
    let mut rng = RandomNumberGenerator::new();

    let delta = match move_intent.movement_type {
        Movement::Random => randomized_move_delta(&mut rng),
        Movement::Keyboard(key) => get_keyboard_delta(key),
    };

    if delta.is_none() {
        return;
    }

    let destination = move_intent.current + delta.unwrap();

    if map.can_enter_cell(destination) {
        commands.add_component(move_intent.entity, destination); // Moves component to new position

        let should_focus_camera = match move_intent.movement_type {
            Movement::Keyboard(_) => true,
            _ => false,
        };

        if should_focus_camera {
            camera.center(destination); // Center around player
        }
    }
    commands.remove(*entity); // Clear intent after it was processed
}
