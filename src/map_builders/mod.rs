use crate::prelude::*;

mod automata;
mod base;
mod drunkard;
mod random_rooms;
mod vaults;

use crate::map_builders::vaults::apply_fortress_vault;
pub use automata::*;
pub use base::*;
pub use drunkard::*;
pub use random_rooms::*;

pub fn create_random_architect(rand: &mut RandomNumberGenerator) -> Box<dyn MapArchitect> {
    let mut architect: Box<dyn MapArchitect> = match rand.range(0, 3) {
        0 => Box::new(DrunkardArchitect::new()),
        1 => Box::new(RandomRoomsArchitect::new()),
        _ => Box::new(CellularAutomataArchitect::new()),
    };

    architect.build(rand);

    let monsters_positions = architect.monsters_positions(rand);

    architect.get_mut_map_builder().spawned_monsters = monsters_positions;

    apply_fortress_vault(&mut architect, rand);

    architect
}
