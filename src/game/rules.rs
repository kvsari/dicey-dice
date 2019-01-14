//! Game rules. Controls what are valid moves.

use crate::hexagon::{
    coordinate,
    grid,
};

use super::{
    tree::Move,
    hold::Hold,
    player::Player,
};

use super::Grid;

fn all_legal_moves_from(grid: &Grid, player: Player) -> Vec<Move> {
    grid.iter()
        .fold(vec![Move::Pass], |mut moves, hex_tile| {
            let coordinate = *hex_tile.coordinate();
            let hold = *hex_tile.data();
            moves.extend(
                coordinate
                    .neighbours()
                    .iter()
                    .filter_map(|neighbour| {
                        grid.fetch(neighbour)
                            .ok() // Ignore the misses
                            .and_then(|d| {
                                if d.owner() != &player {
                                    // We have an enemy tile. We count dice.
                                    if d.dice() < hold.dice() {
                                        // Player has more dice! This is an attacking move.
                                        Some(Move::Attack(coordinate, *neighbour))
                                    } else {
                                        // Player doesn't have enough dice. Can't attack.
                                        None
                                    }
                                } else {
                                    // Our tile is owned by the player. No move here.
                                    None
                                }
                            })
                    })
            );

            moves
        })
}

/// Generates a new grid that bears the consequences of the supplied movement. Doesn't
/// check if the move is legal.
fn grid_from_move(grid: &Grid, movement: Move) -> Grid {
    match movement {
        Move::Pass => grid.to_owned(),
        Move::Attack(from, to) => attacking_move(grid, from, to),
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
