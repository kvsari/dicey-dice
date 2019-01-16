//! Game rules. Controls what are valid moves.

use crate::hexagon::{
    coordinate,
    grid,
};

use super::{
    tree::{Move, BoardState, Consequence, Next},
    hold::Hold,
    player::{Player, Players},
};

use super::Grid;

fn boardstate_consequences(boardstate: &BoardState, players: &mut Players) -> Vec<Next> {
    let moves = all_legal_moves_from(boardstate.grid(), boardstate.player());

    // If there's only the passing move, we check for winner or loser.
    if moves.len() == 1 {
        let action = moves[0];
        
        if winner(boardstate) {            
            return vec![Next::new(action, Consequence::Winner)];
        }

        if loser(boardstate) {
            let new_grid = grid_from_move(boardstate.grid(), action);
            let new_board = BoardState::new(players.next(), new_grid);
            return vec![Next::new(action, Consequence::GameOver(new_board))];
        }
    }

    // Otherwise we continue

    /*
    moves
        .into_iter()
        .map(|action| {
            
        })
        .count();
     */

    // TODO: Finish me
    Vec::new()
}

/// Iterates through the entire board to see if they are all owned by the current player
/// in the `BoardState`. If so, we have a winner. This function should only be called when
/// there are no attacking moves possible from the same `BoardState` being fed in.
fn winner(boardstate: &BoardState) -> bool {
    let player = *boardstate.player();
    boardstate
        .grid()
        .iter()
        .try_for_each(|ht| {
            if ht.data().owner() == &player {
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}

/// A repeat of `winner` above. Should be able to check for either within the same iter.
fn loser(boardstate: &BoardState) -> bool {
    let player = *boardstate.player();
    boardstate
        .grid()
        .iter()
        .try_for_each(|ht| {
            if ht.data().owner() != &player {
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}

fn all_legal_moves_from(grid: &Grid, player: &Player) -> Vec<Move> {
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
                                if d.owner() != player {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn winner_wins() {
        let mut players = Players::new(2);
        let grid = Grid::generate(100, 100, Hold::new(players.current(), 1));

        let board = BoardState::new(players.current(), grid);

        assert!(winner(&board));
    }

    #[test]
    fn loser_loses() {
        let mut players = Players::new(2);
        let grid = Grid::generate(100, 100, Hold::new(players.current(), 1));

        let board = BoardState::new(players.next(), grid);

        assert!(loser(&board));
    }
}
