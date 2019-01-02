//! The tree containing all the legal moves in a game.

use crate::grid::Rectangular;
use crate::hold::Hold;

/// A turn with indexes to other turns that represent valid moves on the game board.
pub struct Turn {
    id: u32,
   
    /// Active player that can move in this turn.
    player: u32,

    /// The game state in this turn.    
    state: Rectangular<Hold>,

    /// The indexes to valid turns from this turn.
    moves: Vec<u32>,
}
