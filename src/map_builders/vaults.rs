use crate::prelude::*;

const FORTRESS_VAULT: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

// Applies a pre-defined vault layout upon current map
pub fn apply_fortress_vault(
    architect: &mut Box<dyn MapArchitect>,
    rand: &mut RandomNumberGenerator,
) {
    let starting_point = architect.get_starting_point();
    let amulet_point = architect.get_amulet_point();
    let map_builder = architect.get_mut_map_builder();

    let placement = get_fortress_placement(map_builder, starting_point, amulet_point, rand);

    // Draw fortress upon placement if available.
    if let Some(placement) = placement {
        draw_vault(map_builder, placement);
    }
}

fn draw_vault(map_builder: &mut MapBuilder, placement: Point) {
    let string_vec: Vec<char> = FORTRESS_VAULT
        .0
        .chars()
        .filter(|a| *a != '\r' && *a != '\n')
        .collect();
    let mut i = 0;

    for ty in placement.y..placement.y + FORTRESS_VAULT.2 {
        for tx in placement.x..placement.x + FORTRESS_VAULT.1 {
            let idx = position_index(Point::new(tx, ty));
            let cell_char = string_vec[i];
            match cell_char {
                'M' => {
                    map_builder.map.cells[idx] = CellType::Floor;
                    map_builder.spawned_monsters.push(Point::new(tx, ty));
                }
                '-' => map_builder.map.cells[idx] = CellType::Floor,
                '#' => map_builder.map.cells[idx] = CellType::Wall,
                _ => println!("No idea what to do with [{}]", cell_char),
            }
            i += 1;
        }
    }
}

fn get_fortress_placement(
    map_builder: &mut MapBuilder,
    starting_point: Point,
    amulet_point: Point,
    rand: &mut RandomNumberGenerator,
) -> Option<Point> {
    let mut placement = None;
    let mut attempts = 0;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[map_builder.map.point2d_to_index(starting_point)],
        &map_builder.map,
        1024.0,
    );

    while placement.is_none() && attempts < 10 {
        let dimensions = get_fortress_dimensions(rand);

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = map_builder.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < MAP_EDGE_PERIMETER && distance > 20.0 && amulet_point != pt {
                can_place = true;
            }
        });

        if can_place {
            // Removes any preliminary spawned monsters from Vault location
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            map_builder
                .spawned_monsters
                .retain(|pt| !points.contains(pt));
        }
        attempts += 1;
    }

    placement
}

fn get_fortress_dimensions(rand: &mut RandomNumberGenerator) -> Rect {
    Rect::with_size(
        rand.range(0, SCREEN_WIDTH - FORTRESS_VAULT.1),
        rand.range(0, SCREEN_HEIGHT - FORTRESS_VAULT.2),
        FORTRESS_VAULT.1,
        FORTRESS_VAULT.2,
    )
}
