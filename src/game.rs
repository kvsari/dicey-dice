//! Game code.
use std::iter::Iterator;

use rand::prelude::*;

use crate::hexagon;

pub mod player;
pub mod tree;
pub mod hold;
mod rules;

use crate::hexagon::coordinate::Cube;

type Grid = hexagon::grid::Rectangular<hold::Hold>;

pub use self::hold::Hold;
pub use self::player::{Player, Players};

pub fn generate_random_grid(columns: u32, rows: u32, players: Players) -> Grid {
    let mut rng = thread_rng();
    let grid = Grid::generate(columns, rows, hold::Hold::default());

    grid.fork_with(move |_,_| {
        let player_dice = rng.gen_range(1, 6);
        hold::Hold::new(players.sample(&mut rng), player_dice)
    })
}

/// Board where player A has no attacking moves and will lose.
pub fn canned_2x2_start01() -> tree::BoardState {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 2)),
        (Cube::from((1, 0)), Hold::new(player2, 3)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let mut grid: Grid = hexes.into_iter().collect();
    grid.set_columns_and_rows(2, 2);
    tree::BoardState::new(players, grid)
}

/// Board where player A has one attacking move.
pub fn canned_2x2_start02() -> tree::BoardState {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 2)),
        (Cube::from((1, 0)), Hold::new(player2, 1)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let mut grid: Grid = hexes.into_iter().collect();
    grid.set_columns_and_rows(2, 2);
    tree::BoardState::new(players, grid)
}

/// Board where player A has two attacking moves.
pub fn canned_2x2_start03() -> tree::BoardState {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 4)),
        (Cube::from((1, 0)), Hold::new(player2, 3)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let mut grid: Grid = hexes.into_iter().collect();
    grid.set_columns_and_rows(2, 2);
    tree::BoardState::new(players, grid)
}
