//! The tree containing all the legal moves in a game.
use std::collections::HashMap;

use crate::hexagon::{coordinate, grid};
use super::{Grid, hold::Hold};

type FromHex = coordinate::Cube;
type ToHex = coordinate::Cube;
type StateIndex = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BoardState {
    player: u8,
    grid: Grid,
}

/// A legal move from one `BoardState` into another.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Attack(FromHex, ToHex),
    Pass,
}

/// What follows from a `Move`.
#[derive(Debug, Clone)]
pub enum Conseqence {
    Continue(BoardState),
    TurnOver(BoardState),
    GameOver(BoardState),
}

#[derive(Debug, Clone)]
pub struct Next {
    movement: Move,
    consequence: Conseqence,
}

/*
/// A traversal through the tree.
pub struct Traversal {
}
 */

/// The game tree. Contains all moves possible from the starting state.
#[derive(Debug, Clone)]
pub struct Tree {
    players: u8,
    start: BoardState,
    traversal: Vec<BoardState>,
    states: HashMap<BoardState, Vec<Next>>,
}

