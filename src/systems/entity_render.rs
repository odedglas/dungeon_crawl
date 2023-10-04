use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera, #[resource] _map: &Map) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let offset = Point::new(camera.left, camera.top);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
