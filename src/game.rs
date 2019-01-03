//! Game code.
use rand::prelude::*;

pub mod hold;

use crate::hexagon;

/// Temporary function. Remove this with a properly wrapped up game tree. An intermediate
/// step is to return a game turn that will wrap the `hold::Hold` allowing that sub-module
/// to be made private.
pub fn generate_random_2x2_board_game() -> hexagon::grid::Rectangular<hold::Hold> {
    let mut rng = thread_rng();    
    let grid = hexagon::grid::Rectangular::generate(2, 2, hold::Hold::default());
    
    grid.fork(move |_| {
        let player_code = rng.gen_range(1, 3);
        let player_dice = rng.gen_range(1, 6);
        hold::Hold::new(player_code, player_dice)
    })
}
