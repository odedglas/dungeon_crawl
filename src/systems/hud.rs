use crate::prelude::*;
use object_pool::Reusable;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(CarriedItem)]
#[read_component(EntityName)]
pub fn hud(ecs: &SubWorld, #[resource] level: &Level) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();

    draw_hud_title(&mut draw_batch, player_health);
    draw_player_health(&mut draw_batch, player_health);
    draw_map_level(&mut draw_batch, *level);
    draw_player_items(ecs, &mut draw_batch);

    draw_batch.submit(10000).expect("Batch error");
}

fn draw_player_items(ecs: &SubWorld, draw_batch: &mut Reusable<DrawBatch>) {
    let mut player_items = <(&Item, &EntityName, &CarriedItem)>::query();

    let mut has_items = false;
    let mut y = 3;
    player_items.iter(ecs).for_each(|(_, name, _)| {
        draw_batch.print(Point::new(3, y + 1), format!("{}. {}", y - 2, name.0));
        y += 1;
        has_items = true;
    });

    if has_items {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried (activate using [NUM])",
            ColorPair::new(YELLOW, BLACK),
        );
    }
}

fn draw_player_health(draw_batch: &mut Reusable<DrawBatch>, player_health: &Health) {
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
}

fn draw_map_level(draw_batch: &mut Reusable<DrawBatch>, level: usize) {
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 2),
        format!("Dungeon Level: {level}"),
        ColorPair::new(YELLOW, BLACK),
    );
}

fn draw_hud_title(draw_batch: &mut Reusable<DrawBatch>, player_health: &Health) {
    draw_batch.print_centered(2, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
}
