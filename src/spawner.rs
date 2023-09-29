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

pub fn spawn_monster(ecs: &mut World, rand: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Monster,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: randomized_monster(rand),
        },
    ));
}

fn randomized_monster(rand: &mut RandomNumberGenerator) -> FontCharType {
    match rand.range(0, 4) {
        0 => to_cp437('E'),
        1 => to_cp437('O'),
        2 => to_cp437('o'),
        _ => to_cp437('g'),
    }
}
