use crate::prelude::*;

#[system]
#[read_component(Point)]
pub fn turn_transition(_: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
    let new_state = turn_state.transition();

    if *turn_state == new_state {
        return;
    }

    *turn_state = new_state;
}
