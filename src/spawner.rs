use crate::prelude::*;

type MonsterSpawn = (i32, String, FontCharType, i32);

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player_component = (
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(GameEntity::Player.glyph()),
        },
        FieldOfView::new(8),
        Health::new(10),
        Damage(1),
    );

    ecs.push(player_component);
}

pub fn spawn_monster(ecs: &mut World, rand: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, damage) = randomized_monster(rand);

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
        Damage(damage),
    ));
}

fn randomized_monster(rand: &mut RandomNumberGenerator) -> MonsterSpawn {
    match rand.roll_dice(1, 10) {
        0..=1 => (
            3,
            "Two Headed".to_string(),
            to_cp437(GameEntity::TwoHeaded.glyph()),
            3,
        ),
        2..=3 => (4, "Oger".to_string(), to_cp437(GameEntity::Oger.glyph()), 2),
        4..=6 => (2, "Orc".to_string(), to_cp437(GameEntity::Orc.glyph()), 2),
        _ => (
            1,
            "Goblin".to_string(),
            to_cp437(GameEntity::Goblin.glyph()),
            1,
        ),
    }
}

pub fn spawn_map_item(ecs: &mut World, entity: &GameEntity, pos: Point) {
    let (display, glyph) = entity.display();

    let components = (
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(glyph),
        },
        EntityName(display),
    );

    let map_item = ecs.push(components);
    if let Some(mut entry) = ecs.entry(map_item) {
        match entity {
            GameEntity::AmuletOfYala => entry.add_component(AmuletOfYala),
            GameEntity::HealingPotion(heal_amount) => entry.add_component(HealingPotion {
                heal_amount: *heal_amount,
            }),
            GameEntity::MapRevealer => entry.add_component(MapRevealer),
            GameEntity::RustySword | GameEntity::ShinySword | GameEntity::HugeSword => {
                entry.add_component(Weapon(entity.get_damage()));
            }
            _ => println!("Invalid Map Item"),
        };
    }
}
