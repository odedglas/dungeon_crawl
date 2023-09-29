use crate::prelude::*;

fn get_mouse_move_delta(key: &Option<VirtualKeyCode>) -> Option<Point> {
    let mut delta: Option<Point> = None;

    if let Some(key) = key {
        delta = match key {
            VirtualKeyCode::Left => Some(Point::new(-1, 0)),
            VirtualKeyCode::Right => Some(Point::new(1, 0)),
            VirtualKeyCode::Up => Some(Point::new(0, -1)),
            VirtualKeyCode::Down => Some(Point::new(0, 1)),
            _ => None,
        };
    }

    delta
}

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(delta) = get_mouse_move_delta(key) {
        let mut players = <&mut Point>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|pos| {
            let destination = *pos + delta;

            if map.can_enter_cell(destination) {
                *pos = destination;
                camera.center(destination);
            }
        });
    }
}
