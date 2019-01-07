//! The tree containing all the legal moves in a game.

use crate::hexagon::{coordinate, grid};
use super::hold::Hold;

type FromHex = coordinate::Cube;
type ToHex = coordinate::Cube;
type StateIndex = usize;

/// A legal move from one `State` into another.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Attack(FromHex, ToHex),
    Pass,
}

/// A particular state of the game. Contains the state of the gird, which player can move
/// and the legal moves available (or not).
#[derive(Debug, Clone)]
pub struct  State {
    id: u32,
   
    /// The player which is permitted to move in this `State`.
    player: u32,

    /// The game state in this turn.    
    state: grid::Rectangular<Hold>,

    /// Valid moves from this `State`. They lead on to further `State`s. A vector of zero
    /// `Move`s indicates that the game has ended and this is the final `State` in this
    /// particular sequence of decisions.
    moves: Vec<(Move, StateIndex)>,
}

/// A turn in the game.
pub struct Turn {
    count: u32,
    state: u32,
}

/*
/// A traversal through the tree.
pub struct Traversal {
}
 */

/// The game tree. Contains all the moves possible from the starting state (index 0).
#[derive(Debug, Clone)]
pub struct Tree {
    /// The leaves/nodes of the tree. We store all the data in an array and index into it
    /// to avoid cycles nested within data structures. The starting state is at index 0.
    /// There will **always** be at least the starting state.
    states: Vec<State>,

    /// Index into the state vector.
    current_state: u32,

    /*
    /// Which turn we're at.
    turn_count: u32,

    /// The index to the turn state.
    turn_state: u32,
    */

    /// The players in the game. They are traversed in the order stored.
    players: Vec<u32>,
}
