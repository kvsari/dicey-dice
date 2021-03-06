//! Game code.
use std::iter::Iterator;

use rand::prelude::*;

use crate::hexagon::{Rectangular, Grid, Cube};

pub mod player;
pub mod model;
mod generate;
mod rules;
mod score;

pub use model::{Board, Tree, Choice, Action, Consequence, Score, Holding};
pub use player::{Player, Players};
pub use generate::{
    start_tree_horizon_limited,
    start_tree_insert_budgeted,
    grow_tree_horizon_limited,
    build_tree
};
pub use score::{score_tree, clear_all_scoring, clear_scoring_from, score_tree_from};
use model::Hold;

pub fn generate_random_grid(columns: u32, rows: u32, players: Players) -> Grid<u8> {
    let mut rng = thread_rng();
    let grid: Grid<u8> = Rectangular::generate(columns, rows, 0).into();

    grid.fork_with(move |_,_| {
        let player_dice = rng.gen_range(1, 6);
        u8::new(players.sample(&mut rng), player_dice, true)
    })
}

pub fn generate_random_board(columns: u32, rows: u32, players: Players) -> Board {
    let grid = generate_random_grid(columns, rows, players);
    Board::new(players, grid, 0, 0)
}

/// Used for testing edge cases more than anything else.
pub fn canned_1x1_start() -> Board {
    let player1 = Player::new(1, 'A');
    let players = Players::new(2);
    let hexes: Vec<(Cube, u8)> = vec![
        ((0, 0).into(), u8::new(player1, 2, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    let grid = grid.change_to_rectangle(1, 1);
    Board::new(players, grid, 0, 0)
}

/// Single line board more for testing purposes than actual play. Player 'A' is destined
/// to lose.
pub fn canned_2x1_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 1), 0, 0)
}

/// Game is an instant stalemate.
pub fn canned_2x1_start02() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 1, true)),
        (Cube::from((1, 0)), u8::new(player2, 1, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 1), 0, 0)
}

/// Game is a clear winner for player 'A'
pub fn canned_2x1_start03() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 5, true)),
        (Cube::from((1, 0)), u8::new(player1, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 1), 0, 0)
}

/// Single line board more for testing purposes than actual play.
pub fn canned_3x1_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((2, 0)), u8::new(player1, 3, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 1), 0, 0)
}

/// Clear winner!
pub fn canned_3x1_start02() -> Board {
    let players = Players::new(2);
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player2, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((2, 0)), u8::new(player2, 3, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 1), 0, 0)
}

/// Instant stalemate
pub fn canned_3x1_start03() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 1, true)),
        (Cube::from((1, 0)), u8::new(player2, 1, true)),
        (Cube::from((2, 0)), u8::new(player1, 1, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 1), 0, 0)
}

/// 3 player stalemate
pub fn canned_3x1_start04() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let player3 = Player::new(3, 'C');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 1, true)),
        (Cube::from((1, 0)), u8::new(player2, 1, true)),
        (Cube::from((2, 0)), u8::new(player3, 1, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 1), 0, 0)
}

/// 3 player game
pub fn canned_3x1_start05() -> Board {
    let players = Players::new(3);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let player3 = Player::new(3, 'C');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((2, 0)), u8::new(player3, 3, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 1), 0, 0)
}

/// Board where player A has no attacking moves and will lose.
pub fn canned_2x2_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((0, 1)), u8::new(player2, 3, true)),
        (Cube::from((1, 1)), u8::new(player2, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0, 0)
}

/// Board where player A has one attacking move.
pub fn canned_2x2_start02() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 2, true)),
        (Cube::from((1, 0)), u8::new(player2, 1, true)),
        (Cube::from((0, 1)), u8::new(player2, 3, true)),
        (Cube::from((1, 1)), u8::new(player2, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0, 0)
}

/// Board where player A has two attacking moves.
pub fn canned_2x2_start03() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 4, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((0, 1)), u8::new(player2, 3, true)),
        (Cube::from((1, 1)), u8::new(player2, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0, 0)
}

/// Board where Player A and Player B will battle. Player A can win, but if he makes a
/// mistake, player B can win instead.
pub fn canned_2x2_start04() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 5, true)),
        (Cube::from((1, 0)), u8::new(player1, 4, true)),
        (Cube::from((0, 1)), u8::new(player2, 5, true)),
        (Cube::from((1, 1)), u8::new(player2, 3, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(2, 2), 0, 0)
}

pub fn canned_3x2_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 4, true)),
        (Cube::from((1, 0)), u8::new(player2, 4, true)),
        (Cube::from((2, 0)), u8::new(player1, 4, true)),
        (Cube::from((0, 1)), u8::new(player2, 5, true)),
        (Cube::from((1, 1)), u8::new(player1, 5, true)),
        (Cube::from((2, 1)), u8::new(player2, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 2), 0, 0)
}

/// Anything higher than this exposes an inneficiency in the movement scoring algorithm.
pub fn canned_3x2_start02() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 3, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((2, 0)), u8::new(player1, 3, true)),
        (Cube::from((0, 1)), u8::new(player2, 3, true)),
        (Cube::from((1, 1)), u8::new(player1, 4, true)),
        (Cube::from((2, 1)), u8::new(player2, 4, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 2), 0, 0)
}

/// A more serious board that consumes quite some resources but can be evaluated.
pub fn canned_3x3_start01() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player2, 3, true)),
        (Cube::from((1, 0)), u8::new(player1, 3, true)),
        (Cube::from((2, 0)), u8::new(player1, 3, true)),
        (Cube::from((0, 1)), u8::new(player2, 2, true)),
        (Cube::from((1, 1)), u8::new(player1, 5, true)),
        (Cube::from((2, 1)), u8::new(player2, 3, true)),
        (Cube::from((-1, 2)), u8::new(player1, 2, true)),
        (Cube::from((0, 2)), u8::new(player1, 5, true)),
        (Cube::from((1, 2)), u8::new(player2, 1, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 3), 0, 0)
}

/// Board where player A is one move away from entering a stalemate with player B.
pub fn canned_3x3_start02() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player2, 1, true)),
        (Cube::from((1, 0)), u8::new(player2, 1, true)),
        (Cube::from((2, 0)), u8::new(player1, 1, true)),
        (Cube::from((0, 1)), u8::new(player1, 2, true)),
        (Cube::from((1, 1)), u8::new(player1, 1, true)),
        (Cube::from((2, 1)), u8::new(player1, 1, true)),
        (Cube::from((-1, 2)), u8::new(player1, 2, true)),
        (Cube::from((0, 2)), u8::new(player1, 5, true)),
        (Cube::from((1, 2)), u8::new(player1, 1, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 3), 0, 0)
}

pub fn canned_3x3_start03() -> Board {
    let players = Players::new(2);
    let player1 = Player::new(1, 'A');
    let player2 = Player::new(2, 'B');
    let hexes = vec![
        (Cube::from((0, 0)), u8::new(player1, 3, true)),
        (Cube::from((1, 0)), u8::new(player2, 3, true)),
        (Cube::from((2, 0)), u8::new(player1, 3, true)),
        (Cube::from((0, 1)), u8::new(player2, 4, true)),
        (Cube::from((1, 1)), u8::new(player1, 4, true)),
        (Cube::from((2, 1)), u8::new(player2, 4, true)),
        (Cube::from((-1, 2)), u8::new(player1, 5, true)),
        (Cube::from((0, 2)), u8::new(player2, 5, true)),
        (Cube::from((1, 2)), u8::new(player1, 5, true)),
    ];
    let grid: Grid<u8> = hexes.into_iter().collect();
    Board::new(players, grid.change_to_rectangle(3, 3), 0, 0)
}
