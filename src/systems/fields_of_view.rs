use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fields_of_view(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut fov = <(Entity, &Point, &mut FieldOfView)>::query();

    // Calculates each of them their current field of view base don their position
    fov.iter_mut(ecs)
        .filter(|(_, _, fov)| fov.is_dirty)
        .for_each(|(_entity, pos, fov)| {
            fov.set_fields_of_view(field_of_view_set(*pos, fov.radius, map));
        });
}
