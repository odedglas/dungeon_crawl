use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player_component = (
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    );

    ecs.push(player_component);
}
