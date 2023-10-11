use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    for y in camera.top..camera.bottom {
        // Uses Camera boundaries to render only what's visible
        for x in camera.left..camera.right {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left, camera.top);

            // Skips if not in bounds
            if !map.in_bounds(point) {
                continue;
            }

            let index = position_index(point);
            let within_player_fov = player_fov.visible_tiles.contains(&point);

            // Skip if item is either not visible or wasn't been discovered already
            if !within_player_fov && !map.revealed_cells[index] {
                continue;
            }

            let cell_tint = if within_player_fov { WHITE } else { DARK_GRAY };

            // Draw map cells
            let cell_type = map.cells[index];
            let glyph = theme.tile_to_render(cell_type);

            draw_batch.set(
                point - offset, // Position would be relative to Camera's offset
                ColorPair::new(cell_tint, BLACK),
                glyph,
            );
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
