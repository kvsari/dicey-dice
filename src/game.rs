//! Game code.
use rand::prelude::*;

use crate::hexagon;

pub mod player;
pub mod tree;
pub mod hold;
mod rules;

type Grid = hexagon::grid::Rectangular<hold::Hold>;

pub use self::player::Players;

pub fn generate_random_grid(columns: u32, rows: u32, players: Players) -> Grid {
    let mut rng = thread_rng();
    let grid = Grid::generate(columns, rows, hold::Hold::default());

    grid.fork_with(move |_,_| {
        let player_dice = rng.gen_range(1, 6);
        hold::Hold::new(players.sample(&mut rng), player_dice)
    })
}
