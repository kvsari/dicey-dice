//! Game code.
use std::iter::Iterator;

use rand::prelude::*;

use crate::hexagon::{Rectangular, Grid, Cube};

pub mod player;
pub mod model;
mod rules;

pub use model::{Board, Tree};
pub use player::{Player, Players};
use model::Hold;

pub fn generate_random_grid(columns: u32, rows: u32, players: Players) -> Grid<Hold> {
    let mut rng = thread_rng();
    let grid: Grid<Hold> = Rectangular::generate(columns, rows, Hold::default()).into();

    grid.fork_with(move |_,_| {
        let player_dice = rng.gen_range(1, 6);
        Hold::new(players.sample(&mut rng), player_dice)
    })
}

pub fn generate_random_board(columns: u32, rows: u32, players: Players) -> Board {
    let grid = generate_random_grid(columns, rows, players);
    Board::new(players, grid, 0)
}

/// Board where player A has no attacking moves and will lose.
pub fn canned_2x2_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 2)),
        (Cube::from((1, 0)), Hold::new(player2, 3)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let grid: Grid<Hold> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0)
}

/// Board where player A has one attacking move.
pub fn canned_2x2_start02() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 2)),
        (Cube::from((1, 0)), Hold::new(player2, 1)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let grid: Grid<Hold> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0)
}

/// Board where player A has two attacking moves.
pub fn canned_2x2_start03() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), Hold::new(player1, 4)),
        (Cube::from((1, 0)), Hold::new(player2, 3)),
        (Cube::from((0, 1)), Hold::new(player2, 3)),
        (Cube::from((1, 1)), Hold::new(player2, 5)),
    ];
    let grid: Grid<Hold> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0)
}
