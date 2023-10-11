use crate::prelude::*;

mod automata;
mod base;
mod drunkard;
mod random_rooms;
mod themes;
mod vaults;

use crate::map_builders::vaults::apply_fortress_vault;
pub use automata::*;
pub use base::*;
pub use drunkard::*;
pub use random_rooms::*;
pub use themes::*;

pub fn create_map_architect(rand: &mut RandomNumberGenerator) -> Box<dyn MapArchitect> {
    let mut architect: Box<dyn MapArchitect> = match rand.range(0, 3) {
        0 => Box::new(DrunkardArchitect::new()),
        1 => Box::new(RandomRoomsArchitect::new()),
        _ => Box::new(CellularAutomataArchitect::new()),
    };

    architect.build(rand);
    architect.set_monsters(rand);

    apply_fortress_vault(&mut architect, rand);

    architect
}
