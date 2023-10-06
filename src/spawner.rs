use crate::prelude::*;

type MonsterSpawn = (i32, String, FontCharType);

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player_component = (
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        FieldOfView::new(8),
        Health::new(10),
    );

    ecs.push(player_component);
}

pub fn spawn_monster(ecs: &mut World, rand: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = randomized_monster(rand);

    ecs.push((
        Monster,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MoveTowardsPlayer,
        FieldOfView::new(6),
        Health::new(hp),
        EntityName(name),
    ));
}

fn randomized_monster(rand: &mut RandomNumberGenerator) -> MonsterSpawn {
    match rand.roll_dice(1, 10) {
        0..=1 => (3, "Two Headed".to_string(), to_cp437('E')),
        2..=3 => (4, "Oger".to_string(), to_cp437('O')),
        4..=6 => (2, "Orc".to_string(), to_cp437('o')),
        _ => (1, "Goblin".to_string(), to_cp437('g')),
    }
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        EntityName("Amulet of Yala".to_string()),
    ));
}
