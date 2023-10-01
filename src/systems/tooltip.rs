use crate::prelude::*;

fn get_entity_display(ecs: &SubWorld, entity: &Entity, name: &str) -> String {
    if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
        format!("{}: {}HP", name, health.current)
    } else {
        name.to_string()
    }
}

#[system]
#[read_component(Point)]
#[read_component(EntityName)]
#[read_component(Health)]
pub fn tooltip(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let offset = Point::new(camera.left, camera.top); // Camera offset
    let map_position = *mouse_pos + offset;

    <(&EntityName, Entity, &Point)>::query()
        .iter(ecs)
        .filter(|(_, _, pos)| **pos == map_position)
        .for_each(|(name, entity, _)| {
            let screen_position = *mouse_pos * 4; // Scale up to match fontsize
            let display = get_entity_display(ecs, entity, &name.0);

            draw_batch.print(screen_position, display);
        });

    draw_batch.submit(5000).expect("Batch error");
}
