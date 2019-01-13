//! Game code.
use rand::prelude::*;

use crate::hexagon;

pub mod player;
pub mod tree;
pub mod hold;
mod rules;

type Grid = hexagon::grid::Rectangular<hold::Hold>;

/// Temporary function. Remove this with a properly wrapped up game tree. An intermediate
/// step is to return a game turn that will wrap the `hold::Hold` allowing that sub-module
/// to be made private.
pub fn generate_random_2x2_board_game() -> hexagon::grid::Rectangular<hold::Hold> {
    let mut rng = thread_rng();    
    let grid = hexagon::grid::Rectangular::generate(2, 2, hold::Hold::default());
    
    grid.fork_with(move |_, _| {
        let player_code = rng.gen_range(1, 3);
        let player_dice = rng.gen_range(1, 6);
        hold::Hold::new(player_code, player_dice)
    })
}

fn generate_random_rectangular_board(
    columns: u32, rows: u32, players: u8
) -> hexagon::grid::Rectangular<hold::Hold> {
    let mut rng = thread_rng();
    hexagon::grid::Rectangular::generate_with(
        columns,
        rows,
        |_cube| {
            let player_code = rng.gen_range(1, players + 1);
            let player_dice = rng.gen_range(1, 6);
            hold::Hold::new(player_code, player_dice)
        },
    )
}
