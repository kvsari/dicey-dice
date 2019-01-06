//! Game rules. Controls what are valid moves.

use crate::hexagon::{coordinate, grid};
use super::{tree, hold};

fn all_legal_moves_from(
    grid: &grid::Rectangular<hold::Hold>, player: u32
) -> Vec<tree::Move> {
    Vec::new()
}

/// Returns a new rectangular grid representing the consequences of the move applied to the
/// sent grid by the specified player. Doesn't check if the move is legal or not.
fn carry_out_move(
    grid: &grid::Rectangular<hold::Hold>,
    decision: tree::Move,
) -> grid::Rectangular<hold::Hold> {
    // TODO: Finish me. There are two sub-functions to write here. One that handles
    //       passing moves and the other which handles attacking moves. Do the passing
    //       moves first as that's the easiest as its a simple clone of the grid.
    grid.to_owned()
}
