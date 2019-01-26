//! Game rules. Controls what are valid moves.
use std::collections::HashMap;

use crate::hexagon::{Grid, Cube};
use super::model::*;
use super::Player;

/// Function will build all boardstates from `start`  inserting them into the `states` map.
/// If the boardstate already exists will skip that boardstate. This function has no
/// horizon so it won't stop generating until the stack is empty.
pub fn calculate_all_consequences(start: Board) -> HashMap<Board, Vec<Choice>> {
    //depth_first_calc_consequences(start)
    let (tree, stats) = breadth_first_calc_consequences(start);

    stats
        .iter()
        .for_each(|stat| println!("{}", stat));

    let totals = stats
        .iter()
        .fold(Totals::default(), |totals, stats| {
            let n_totals = Totals::new(*stats.boards(), *stats.inserted());
            totals + n_totals
        });
    println!("{}", &totals);
    
    tree
}

/// First implementation in calculating the entire game tree. Works by following each
/// branch down to the end before exploring other possibilities.
fn depth_first_calc_consequences(start: Board) -> HashMap<Board, Vec<Choice>> {
    let mut stack: Vec<Board> = Vec::new();
    let mut states: HashMap<Board, Vec<Choice>> = HashMap::new();
    
    // 1. Seed tree generation by pushing the first boardstate onto the stack.
    stack.push(start);

    let mut count = 0;
    // 2. Get the next board off the stack.
    while let Some(board) = stack.pop() {
        // 3. Check if the board hasn't been stored yet. If it has we skip.
        if !states.contains_key(&board) {           
            // 4. If not, we calculate the `Choice`s.
            let choices = choices_from_board(&board);

            // 5. We then push all the resulting boardstates onto the stack.
            stack.extend(
                choices
                    .iter()
                    .map(|choice| choice.consequence().board().to_owned())
            );

            // 6. Then we insert the boardstate and `Choice`s into the map.
            states.insert(board, choices);
        }
        println!("Stack size: {}", &stack.len());
        count += 1;
    }
    println!("Looped {} times. States stored: {}", &count, &states.len());

    // 7. Return results of traversal.
    states
}

/// Calculate all consequences going layer by layer rather than following a single
/// branch all the way to the end and then backtracking upwards. This means that each
/// layer will grow exponentially large but it will be easier to see how the dataset
/// grows geometrically as the grid size/players increase linearly.
fn breadth_first_calc_consequences(
    start: Board
) -> (HashMap<Board, Vec<Choice>>, Vec<LayerStats>) {
    let mut states: HashMap<Board, Vec<Choice>> = HashMap::new();
    let mut current_layer: Option<Vec<Board>> = Some(vec![start]);
    let mut layer_count: usize = 0;
    let mut layer_stats: Vec<LayerStats> = Vec::new();
    
    loop {
        let layer = current_layer.take().unwrap();
        
        if layer.is_empty() {
            break;
        }

        // Prepare some stats.
        layer_count += 1;
        let layer_boards = layer.len();
        let mut board_inserts = 0;
        //
        
        let mut next_layer = Vec::new();
        for board in layer {
            if !states.contains_key(&board) {
                //let choices = choices_from_board(&board);
                let choices = choices_from_board_only_pass_at_end(&board);
                next_layer.extend(
                    choices
                        .iter()
                        .map(|choice| choice.consequence().board().to_owned())
                );
                states.insert(board, choices);

                // Prepare more stats.
                board_inserts += 1;
            }
        }
        current_layer = Some(next_layer);

        // Record the stats.
        layer_stats.push(LayerStats::new(layer_count, layer_boards, board_inserts));
    }

    (states, layer_stats)
}

fn choices_from_board(board: &Board) -> Vec<Choice> {
    let attacking_moves = all_legal_attacks_from(
        board.grid(), &board.players().current()
    );

    // If there are no attacking moves, we quickly check if the player has won or lost.
    if attacking_moves.is_empty() {
        if winner(board) {            
            return vec![Choice::new(Action::Pass, Consequence::Winner(board.to_owned()))];
        }

        if loser(board) {
            let new_grid = grid_from_move(board.grid(), Action::Pass);
            let new_board = Board::new(
                board.players().remove_current(), new_grid, 0
            );
            return vec![Choice::new(Action::Pass, Consequence::GameOver(new_board))];
        }
    }

    // Otherwise we continue.
    let mut choices: Vec<Choice> = attacking_moves
        .into_iter()
        .map(|attack| {
            let new_grid = grid_from_move(board.grid(), attack);
            // TODO: Add dice captures.
            let new_board = Board::new(*board.players(), new_grid, 0);
            Choice::new(attack, Consequence::Continue(new_board))
        })
        .collect();

    // And we tack on the passing move at the end.
    let new_grid = grid_from_move(board.grid(), Action::Pass);
    // TODO: Reinforcement calculations for the passing move.
    let new_board = Board::new(board.players().next(), new_grid, 0);
    choices.push(Choice::new(Action::Pass, Consequence::TurnOver(new_board)));

    choices
}

/// Like `choices_from_board` above but does not add a passing move until there are no
/// attacking moves left. This is to see if it reduces tree generation depth/breadth.
fn choices_from_board_only_pass_at_end(board: &Board) -> Vec<Choice> {
    let attacking_moves = all_legal_attacks_from(
        board.grid(), &board.players().current()
    );

    let mut choices: Vec<Choice> = Vec::new();

    // If there are no attacking moves, we quickly check if the player has won or lost.
    if attacking_moves.is_empty() {
        if winner(board) {            
            return vec![Choice::new(Action::Pass, Consequence::Winner(board.to_owned()))];
        }

        if loser(board) {
            let new_grid = grid_from_move(board.grid(), Action::Pass);
            let new_board = Board::new(
                board.players().remove_current(), new_grid, 0
            );
            return vec![Choice::new(Action::Pass, Consequence::GameOver(new_board))];
        }

        // Since there is not winner or knockout. We add a passing move.
        let new_grid = grid_from_move(board.grid(), Action::Pass);
        // TODO: Reinforcement calculations for the passing move.
        let new_board = Board::new(board.players().next(), new_grid, 0);
        choices.push(Choice::new(Action::Pass, Consequence::TurnOver(new_board)));
    }

    // Process attacking moves. This is functionally skipped if there are none.
    choices.extend(
        attacking_moves
            .into_iter()
            .map(|attack| {
                let new_grid = grid_from_move(board.grid(), attack);
                // TODO: Add dice captures.
                let new_board = Board::new(*board.players(), new_grid, 0);
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
            if ht.data().owner() == &player {
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
            if ht.data().owner() != &player {
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}

/// Produces all legal attacking moves.
fn all_legal_attacks_from(grid: &Grid<Hold>, player: &Player) -> Vec<Action> {
    grid.iter()
        .fold(Vec::new(), |mut moves, hex_tile| {
            //dbg!(hex_tile);
            let coordinate = *hex_tile.coordinate();
            let hold = *hex_tile.data();

            if hold.owner() == player {
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
                                    if d.owner() != player {
                                        // We have an enemy tile. We count dice.
                                        if d.dice() < hold.dice() {
                                            // Player has more dice! 
                                            Some(Action::Attack(coordinate, *neighbour))
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
fn grid_from_move(grid: &Grid<Hold>, movement: Action) -> Grid<Hold> {
    match movement {
        Action::Pass => grid.to_owned(),
        Action::Attack(from, to) => attacking_move(grid, from, to),
    }
}

/// An attacking move that removes all the dice except one from the `from` hexagon and
/// places them minus one to the `to` tile. There is no error checking as this function
/// expects correct parameters to be entered. Thus invalid data will cause a panic.
fn attacking_move(grid: &Grid<Hold>, from: Cube, to: Cube) -> Grid<Hold> {
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
    use crate::hexagon::Rectangular;
    use crate::game::*;
    use super::*;

    #[test]
    fn winner_wins() {
        let players = Players::new(2);
        let grid: Grid<Hold> = Rectangular::generate(
            100, 100, Hold::new(players.current(), 1)
        ).into();

        let board = Board::new(players, grid, 0);

        assert!(winner(&board));
    }

    #[test]
    fn loser_loses() {
        let players = Players::new(2);
        let grid: Grid<Hold> = Rectangular::generate(
            100, 100, Hold::new(players.current(), 1)
        ).into();

        let board = Board::new(players.next(), grid, 0);

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
        let mut choices = choices_from_board(&board);

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
    fn breadth_first_on_canned_2x2_start01() {
        let board = canned_2x2_start01();
        let (states, _stats) = breadth_first_calc_consequences(board);
        assert!(states.len() == 4);
    }

    #[test]
    fn depth_first_on_canned_2x2_start01() {
        let board = canned_2x2_start01();
        let states = depth_first_calc_consequences(board);
        assert!(states.len() == 4);
    }

    /*
    #[test]
    fn count_states_from_canned_2x2_start02() {
        let board = canned_2x2_start02();
        let states = calculate_all_consequences(board);
        assert!(states.len() == 7);
    }
    */
}
