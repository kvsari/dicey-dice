//! Game code.
use std::iter::Iterator;

use rand::prelude::*;

use crate::hexagon;

pub mod player;
pub mod tree;
pub mod hold;
mod rules;

use crate::hexagon::{Rectangular, Grid};
use crate::hexagon::coordinate::Cube;

pub use self::hold::Hold;
pub use self::player::{Player, Players};

type Mesh = Grid<Hold>;

pub fn generate_random_grid(columns: u32, rows: u32, players: Players) -> Mesh {
    let mut rng = thread_rng();
    let grid: Mesh = Rectangular::generate(columns, rows, hold::Hold::default()).into();

    grid.fork_with(move |_,_| {
        let player_dice = rng.gen_range(1, 6);
        Hold::new(players.sample(&mut rng), player_dice)
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
    let grid: Mesh = hexes.into_iter().collect();
    tree::BoardState::new(players, grid.change_to_rectangle(2, 2))
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
    let grid: Mesh = hexes.into_iter().collect();
    tree::BoardState::new(players, grid.change_to_rectangle(2, 2))
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
    let grid: Mesh = hexes.into_iter().collect();
    tree::BoardState::new(players, grid.change_to_rectangle(2, 2))
}
