//! The tree containing all the legal moves in a game.
use std::collections::HashMap;

use crate::hexagon::{coordinate, grid};
use super::{
    Grid,
    hold::Hold,
    player::{Player, Players},
};

type FromHex = coordinate::Cube;
type ToHex = coordinate::Cube;
type StateIndex = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BoardState {
    player: Player,
    grid: Grid,
}

impl BoardState {
    fn new(player: Player, grid: Grid) -> Self {
        BoardState { player, grid }
    }
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
    players: Players,
    start: BoardState,
    traversal: Vec<BoardState>,
    states: HashMap<BoardState, Vec<Next>>,
}

/// Generate a full grame decision free encompassing all possible legal moves starting
/// from the current player returned by `players`.
pub fn grow_entire_tree_from(grid: Grid, players: Players) -> Tree {
    let starting_state = BoardState::new(players.current(), grid);

    // TODO: Finish me.
    Tree {
        players,
        start: starting_state,
        traversal: Vec::new(),
        states: HashMap::new(),
    }
}
