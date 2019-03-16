//! Game rules. Controls what are valid moves.

use crate::hexagon::{Grid, Cube};
use super::model::*;
use super::Player;

/// Maximum amount of dice a hexagon holding may have.
const MAX_DICE: u8 = 5;

/// Calculated all valid moves except the passing move until there are no
/// attacking moves left. This greatly reduces the tree branches.
pub (in crate::game) fn choices_from_board_only_pass_at_end(
    board: &Board, move_limit: u8,
) -> Vec<Choice> {
    let attacking_moves = all_legal_attacks_from(
        board.grid(), &board.players().current()
    );

    let mut choices: Vec<Choice> = Vec::new();
    let moved = *board.moved() + 1;
    //println!("Calculating.... Moved: {}, Limit: {}", &moved, &move_limit);

    // If there are no attacking moves, we quickly check if the player has won or lost.
    if attacking_moves.is_empty() {
        // First we check if there's a winner. This will end the game if so.
        if winner(board) {            
            return vec![Choice::new(Action::Pass, Consequence::Winner(board.to_owned()))];
        }

        // Next we check if the player has been knocked out.
        if loser(board) {
            let new_grid = grid_from_move(board.grid(), Action::Pass);
            let new_board = Board::new(
                board.players().remove_current(), new_grid, 0, 0
            );
            return vec![Choice::new(Action::Pass, Consequence::GameOver(new_board))];
        }

        // Lastly, we check if the game has been locked in a stalemate. This also ends
        // the game but there is no winner. We haven't yet implemented scoring to determine
        // a winner by points or a tie-breaker.
        if stalemate(board) {
            return vec![
                Choice::new(Action::Pass, Consequence::Stalemate(board.to_owned()))
            ];
        }   

        // Since there is not winner or knockout. We add a passing move.
        let new_grid = reinforce02(
            board.grid(), board.players().current(), *board.captured_dice(),
        );
        let new_board = Board::new(board.players().next(), new_grid, 0, 0);
        choices.push(Choice::new(Action::Pass, Consequence::TurnOver(new_board))); 
    } else if moved > move_limit {
        // If we have exceeded the move limit, we pass.
        let new_grid = reinforce02(
            board.grid(), board.players().current(), *board.captured_dice(),
        );
        let new_board = Board::new(board.players().next(), new_grid, 0, 0);
        return vec![Choice::new(Action::Pass, Consequence::TurnOver(new_board))];
    }

    // Process attacking moves. This is functionally skipped if there are none.
    let captured_dice = *board.captured_dice();
    choices.extend(
        attacking_moves
            .into_iter()
            .map(|attack| {
                let new_grid = grid_from_move(board.grid(), attack);
                let total_captured = captured_dice + attack.capturing();
                let new_board = Board::new(
                    *board.players(), new_grid, total_captured, moved,
                );
                Choice::new(attack, Consequence::Continue(new_board))
            })
            .collect::<Vec<Choice>>()
    );

    choices
}

/// Iterates through the entire board to see if they are all owned by the current player
/// in the `BoardState`. If so, we have a winner. This function should only be called when
/// there are no attacking moves possible from the same `BoardState` being fed in.
fn winner(board: &Board) -> bool {
    let player = board.players().current();
    board
        .grid()
        .iter()
        .try_for_each(|ht| {
            if ht.data().owner() == player {
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}

/// A repeat of `winner` above. Should be able to check for either within the same iter.
fn loser(board: &Board) -> bool {
    let player = board.players().current();
    board
        .grid()
        .iter()
        .try_for_each(|ht| {
            if ht.data().owner() != player {
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}

/// Check if the board is in a statelmate condition. This means that there is more than
/// one player and no player can attack another player. Once a stalemate has been detected,
/// then we can layer on calculation as to whether it's a draw or win by points.
fn stalemate(board: &Board) -> bool {
    // Special case for boards consisting of a single or no hex tile. They cant be in
    // stalemate at all, it's impossible.
    if board.grid().len() < 2 {
        return false;
    }
    
    board
        .grid()
        .iter()
        .try_for_each(|ht| {
            let hold = *ht.data();
            let coordinate = *ht.coordinate();
            
            coordinate
                .three_neighbours()
                .iter()
                .try_for_each(|neighbour| {
                    board
                        .grid()
                        .fetch(neighbour)
                        .map(|d| Some(d))
                        .or(Ok(None))
                        .and_then(|maybie| {
                            if let Some(other) = maybie {
                                // Check if the other tile is held by another.
                                if other.owner() != hold.owner() {
                                    // If so, we check if an attack is ever possible. This
                                    // means that either one must be more than 1.
                                    if other.dice() > 1 || hold.dice() > 1 {
                                        // An attack is possible. Short-circuit out.
                                        Err(())
                                    } else {
                                        // An attack is not possible. Continue scanning.
                                        Ok(())
                                    }
                                } else {
                                    // Other tile is held by same owner. Attack impossible.
                                    Ok(())
                                }
                            } else {
                                // No neighbour tile. Impossible to attack.
                                Ok(())
                            }
                        })
                })

        })
        .is_ok()
}

/// Produces all legal attacking moves with the amount of dice they would capture.
fn all_legal_attacks_from(grid: &Grid<u8>, player: &Player) -> Vec<Action> {
    grid.iter()
        .fold(Vec::new(), |mut moves, hex_tile| {
            //dbg!(hex_tile);
            let coordinate = *hex_tile.coordinate();
            let hold = *hex_tile.data();

            if hold.owner() == *player && hold.mobile() {
                moves.extend(
                    coordinate
                        .neighbours()
                        .iter()
                        .filter_map(|neighbour| {
                            //dbg!(neighbour);
                            grid.fetch(neighbour)
                                .ok() // Ignore the misses
                                .and_then(|d| {
                                    //dbg!(d);
                                    if d.owner() != *player {
                                        // We have an enemy tile. We count dice.
                                        if hold.dice() > 1 && d.dice() <= hold.dice() {
                                            // Player has more dice! 
                                            Some(Action::Attack(
                                                coordinate,
                                                *neighbour,
                                                hold.dice(),
                                                d.dice(),
                                            ))
                                        } else {
                                            // Player doesn't have enough dice.
                                            None
                                        }
                                    } else {
                                        // Our tile is owned by the player. No move here.
                                        None
                                    }
                                })
                        })
                );
            }
                
            moves
        })
}

/// Generates a new grid that bears the consequences of the supplied movement. Doesn't
/// check if the move is legal.
fn grid_from_move(grid: &Grid<u8>, movement: Action) -> Grid<u8> {
    match movement {
        Action::Pass => grid.to_owned(),
        Action::Attack(from, to, _, _) => attacking_move(grid, from, to),
    }
}

/// An attacking move that removes all the dice except one from the `from` hexagon and
/// places them minus one to the `to` tile. There is no error checking as this function
/// expects correct parameters to be entered. Thus invalid data will cause a panic.
fn attacking_move(grid: &Grid<u8>, from: Cube, to: Cube) -> Grid<u8> {
    let (to_hold, from_hold) = grid
        .fetch(&from)
        .map(|h| (
            u8::new(h.owner(), h.dice() - 1, h.mobile()),
            u8::new(h.owner(), 1, h.mobile())
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

/*
/// Add reinforcements for the current player on the grid returning a new grid. If
/// there is no space left (a player hex cannot have more than five dice) then any
/// remaining reinforcements are dropped.
///
/// The reinforcements will be doled out super simple. It will simply add them from the
/// top leftmost of any player holdings downwards.
fn reinforce01(grid: &Grid<Hold>, player: Player, reinforcements: u8) -> Grid<Hold> {
    let mut reinforcements = reinforcements;
    grid.fork_with(|_, hold| {
        if hold.owner() == &player {
            let dice = *hold.dice();
            let diff = MAX_DICE - dice;
            let add = if reinforcements > diff {
                reinforcements -= diff;
                diff
            } else {
                let diff = reinforcements;
                reinforcements = 0;
                diff
            };
            Hold::new(player, dice + add)
        } else {
            hold
        }
    })
}
*/

/// Add reinforcements minus 1 for the current player on the grid returning a new grid. If
/// there is no space left (a player hex cannot have more than five dice) then any
/// remaining reinforcements are dropped.
///
/// The reinforcements will be doled out super simple. It will simply add them from the
/// top leftmost of any player holdings downwards.
fn reinforce02(grid: &Grid<u8>, player: Player, reinforcements: u8) -> Grid<u8> {
    let mut reinforcements = reinforcements
        .checked_sub(1)
        .unwrap_or(0);

    grid.fork_with(|_, hold| {
        if hold.owner() == player {
            let dice = hold.dice();
            let diff = MAX_DICE - dice;
            let add = if reinforcements > diff {
                reinforcements -= diff;
                diff
            } else {
                let diff = reinforcements;
                reinforcements = 0;
                diff
            };
            u8::new(player, dice + add, true)
        } else {
            hold
        }
    })
}

#[cfg(test)]
mod test {
    use crate::hexagon::Rectangular;
    use crate::game::*;
    use super::*;

    #[test]
    fn winner_wins() {
        let players = Players::new(2);
        let grid: Grid<u8> = Rectangular::generate(
            100, 100, u8::new(players.current(), 1, true)
        ).into();

        let board = Board::new(players, grid, 0, 0);

        assert!(winner(&board));
    }

    #[test]
    fn loser_loses() {
        let players = Players::new(2);
        let grid: Grid<u8> = Rectangular::generate(
            100, 100, u8::new(players.current(), 1, true)
        ).into();

        let board = Board::new(players.next(), grid, 0, 0);

        assert!(loser(&board));
    }

    #[test]
    fn no_attacking_moves_available() {
        let board = super::super::canned_2x2_start01();
        let attacks = all_legal_attacks_from(board.grid(), &board.players().current());
        
        assert!(attacks.is_empty());
    }

    #[test]
    fn one_attacking_move_available() {
        let board = super::super::canned_2x2_start02();
        let attacks = all_legal_attacks_from(board.grid(), &board.players().current());
        
        assert!(attacks.len() == 1);
    }

    #[test]
    fn two_attacking_moves_available() {
        let board = super::super::canned_2x2_start03();
        let attacks = all_legal_attacks_from(board.grid(), &board.players().current());
        
        assert!(attacks.len() == 2);
    }

    #[test]
    fn test_turn_over() {
        let player2 = Player::new(2, 'B');
        let board = super::super::canned_2x2_start01();
        let mut choices = choices_from_board_only_pass_at_end(&board, 6);

        assert!(choices.len() == 1);
        let choice = choices.pop().unwrap();
        assert!(*choice.action() == Action::Pass);
        let consequence = choice.consequence();
        match consequence {
            Consequence::TurnOver(board) => {
                assert!(board.players().current() == player2);
            },
            _ => panic!("Invalid consequence."),
        }
    }

    #[test]
    fn no_stalemate01() {
        assert!(!stalemate(&canned_1x1_start()));
    }

    #[test]
    fn no_stalemate02() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            ((0, 0).into(), u8::new(player1, 2, true)),
            ((0, 1).into(), u8::new(player2, 1, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let grid = grid.change_to_rectangle(2, 1);
        let board = Board::new(players, grid, 0, 0);

        // Test
        assert!(!stalemate(&board));
    }

    #[test]
    fn no_stalemate03() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            ((0, 0).into(), u8::new(player1, 1, true)),
            ((0, 1).into(), u8::new(player2, 2, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let grid = grid.change_to_rectangle(2, 1);
        let board = Board::new(players, grid, 0, 0);

        // Test
        assert!(!stalemate(&board));
    }

    #[test]
    fn no_stalemate04() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            ((0, 0).into(), u8::new(player1, 2, true)),
            ((0, 1).into(), u8::new(player2, 2, true)),
            ((1, 0).into(), u8::new(player1, 2, true)),
            ((1, 1).into(), u8::new(player2, 2, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let grid = grid.change_to_rectangle(2, 2);
        let board = Board::new(players, grid, 0, 0);

        // Test
        assert!(!stalemate(&board));
    }

    /// This is testing `canned_3x3_start02` and is related to `stalemate04` as that's
    /// one of two possible moves either of which results in a stalemate.
    #[test]
    fn no_stalemate05() {
        // Setup
        let board = crate::game::canned_3x3_start02();

        // Test
        assert!(!stalemate(&board));
    }

    #[test]
    fn stalemate01() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            ((0, 0).into(), u8::new(player1, 1, true)),
            ((0, 1).into(), u8::new(player2, 1, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let grid = grid.change_to_rectangle(2, 1);
        let board = Board::new(players, grid, 0, 0);

        // Test
        assert!(stalemate(&board));
    }

    #[test]
    fn stalemate02() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            ((0, 0).into(), u8::new(player1, 1, true)),
            ((0, 1).into(), u8::new(player2, 1, true)),
            ((1, 0).into(), u8::new(player1, 1, true)),
            ((1, 1).into(), u8::new(player2, 1, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let grid = grid.change_to_rectangle(2, 2);
        let board = Board::new(players, grid, 0, 0);

        // Test
        assert!(stalemate(&board));
    }

    /// The result taking one of the two possible moves from `canned_3x3_start02`. Either
    /// move results in a stalemate.
    #[test]
    fn stalemate04() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            (Cube::from((0, 0)), u8::new(player1, 1, true)),
            (Cube::from((1, 0)), u8::new(player2, 1, true)),
            (Cube::from((2, 0)), u8::new(player1, 1, true)),
            (Cube::from((0, 1)), u8::new(player1, 1, true)),
            (Cube::from((1, 1)), u8::new(player1, 1, true)),
            (Cube::from((2, 1)), u8::new(player1, 1, true)),
            (Cube::from((0, 2)), u8::new(player1, 2, true)),
            (Cube::from((1, 2)), u8::new(player1, 5, true)),
            (Cube::from((2, 0)), u8::new(player1, 1, true)),
        ];
        let grid: Grid<u8> = hexes.into_iter().collect();
        let board = Board::new(players, grid.change_to_rectangle(3, 3), 0, 0);

        // Test
        assert!(stalemate(&board));
    }

    /// The the result of taking both possible moves from `canned_3x1_start01`.
    #[test]
    fn stalemate05() {
        // Setup
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let players = Players::new(2);
        let hexes: Vec<(Cube, u8)> = vec![
            (Cube::from((0, 0)), u8::new(player2, 1, true)),
            (Cube::from((1, 0)), u8::new(player1, 1, true)),
            (Cube::from((2, 0)), u8::new(player1, 2, true)),
        ];

        let grid: Grid<u8> = hexes.into_iter().collect();
        let board = Board::new(players, grid.change_to_rectangle(3, 1), 0, 0);

        // Test
        assert!(stalemate(&board));
    }
}
