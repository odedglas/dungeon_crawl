use crate::prelude::*;

fn has_keypress(key: &Option<VirtualKeyCode>) -> bool {
    if let Some(key) = key {
        return match key {
            VirtualKeyCode::Left => true,
            VirtualKeyCode::Right => true,
            VirtualKeyCode::Up => true,
            VirtualKeyCode::Down => true,
            _ => false,
        };
    }

    false
}

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if has_keypress(key) {
        let mut players = <(Entity, &mut Point)>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|(entity, pos)| {
            // Push move intent processed by movement system
            commands.push((
                (),
                MoveIntent {
                    entity: *entity,
                    current: *pos,
                    movement_type: Movement::Keyboard(key.unwrap()),
                },
            ));
        });

        turn_state.next();
    }
}
