//! Game rules. Controls what are valid moves.

use crate::hexagon::{coordinate, grid};
use super::{tree, hold::Hold};

type Grid = grid::Rectangular<Hold>;

fn all_legal_moves_from(
    grid: &Grid, curr_player: u32, next_player: u32
) -> Vec<(tree::Move, usize)> {
    Vec::new()
}

/// Returns a new rectangular grid representing the consequences of the move applied to the
/// sent grid by the specified player. Doesn't check if the move is legal or not.
fn carry_out_grid_move(grid: &Grid, decision: tree::Move) -> Grid {
    // TODO: Finish me. There are two sub-functions to write here. One that handles
    //       passing moves and the other which handles attacking moves. Do the passing
    //       moves first as that's the easiest as its a simple clone of the grid.
    match decision {
        tree::Move::Pass => grid.to_owned(),
        tree::Move::Attack(from, to) =>  attacking_move(grid, from, to),
    }
}

/// An attacking move that removes all the dice except one from the `from` hexagon and
/// places them minus one to the `to` tile. There is no error checking as this function
/// expects correct parameters to be entered. Thus invalid data will cause a panic.
fn attacking_move(grid: &Grid, from: coordinate::Cube, to: coordinate::Cube) -> Grid {
    let (to_hold, from_hold) = grid
        .fetch(&from)
        .map(|h| (
            Hold::new(*h.owner(), *h.dice() - 1),
            Hold::new(*h.owner(), 1)
        ))
        .expect("Invalid from coordinate.");

    grid.fork_with(|cube, hold| {
        if cube == &from {
            from_hold
        } else if cube == &to {
            to_hold
        } else {
            hold
        }
    })
}
