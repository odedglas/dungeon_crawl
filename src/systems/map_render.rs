use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(0);

    for y in camera.top..camera.bottom {
        // Uses Camera boundaries to render only what's visible
        for x in camera.left..camera.right {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left, camera.top);

            if let Some(index) = safe_position_index(&point) {
                let glyph = match map.cells[index] {
                    CellType::Floor => to_cp437('.'),
                    CellType::Wall => to_cp437('#'),
                };

                draw_batch.set(
                    point - offset, // Position would be relative to Camera's offset
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
