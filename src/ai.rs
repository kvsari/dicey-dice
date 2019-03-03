//! Primitive AI that works on scoring moves in advance and chooses the highest scoring one
//! during play.
use std::collections::HashMap;

use crate::game::{Board, Players, Player, Tree, Action, Consequence, Score};

/// One point/step on the traversal of the game tree.
#[derive(Debug, Clone)]
struct Direction {
    /// Index into the tree and also the current player.
    board: Board,

    /// The choice taken.
    route: usize,

    /// On rewinding, we store the scores.
    scores: HashMap<Player, f64>,
}

impl Direction {
    pub fn new(board: Board, route: usize) -> Self {
        Direction { board, route, scores: HashMap::new() }
    }
}

/// Look at a board and calculate a score from 0 to 1 for all the `Players`. It assumes
/// that the board has already been checked to not be a winning or losing board.
///
/// This will create a score by calculating the percentage of occupied tiles. No further
/// analysis is done.
pub fn score_board01(board: &Board) -> HashMap<Player, f64> {
    let mut count: HashMap<Player, usize> = HashMap::new();
    let tiles = board.grid().len() as f64;
    
    board
        .grid()
        .iter()
        .for_each(|ht| {
            count.entry(*ht.data().owner())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

    count
        .into_iter()
        .map(|(player, held)| {
            let held = held as f64;
            (player, held / tiles)
        })
        .collect()
}

/// Traverse the tree creating a score for each move for the current player. The score will
/// indicate a path to a winning position.
pub fn score_tree(tree: &mut Tree) {
    // Stack
    let mut traversal: Vec<Direction> = Vec::new();

    // Start the traversal
    let board = tree.root().to_owned();
    let mut choices = tree.mut_fetch_choices_unchecked(&board);

    // Special case if the game is over already
    if choices.len() == 1 {
        if *choices[0].action() == Action::Pass {
            if let Consequence::Winner(_) = choices[0].consequence() {
                choices[0].set_score(Score::new(1_f64, 0));
                return;
            }
        }
    }

    let direction = Direction::new(board, 0);
    traversal.push(direction);

    while let Some(ref mut direction) = traversal.last_mut() {
        let mut choices = tree.mut_fetch_choices_unchecked(&direction.board);
        let route = direction.route;
        let curr_player = *direction.board.players().current();
        if choices.len() > route {
            match choices[route].consequence() {
                Consequence::Stalemate(ref board) => {
                    // Game could end here. It's not an ideal end.
                    let scores = score_board01(board);
                    let score = Score::new(*scores.get(&curr_player).unwrap(), 0);
                    choices[route].set_score(score);
                    //drop(choices);
                    scores
                        .into_iter()
                        .for_each(|(player, dest)| {
                            let new_score = Score::new(dest, 0);
                            if let Some(existing_score) = *direction.scores.get(&player) {
                                if existing_score < new_score {
                                    direction.scores.insert(new_score);
                                }
                            } else {
                                direction.scores.insert(new_score)
                            }
                        });
                    direction.route += 1;
                    //drop(direction);
                    continue;
                },
                Consequence::GameOver(ref board) => {
                    // It is game over for the current player. But the game may continue.
                    let score = Score::new(0_f64, 0);
                    choices[route].set_score(score);
                    //drop(choices);
                    direction.scores.insert(curr_player, score); // need to check for existing otherwise risk clobbering a better route.
                    drop(direction);
                    traversal.push(Direction::new(board.to_owned(), 0));
                    continue;
                },
                Consequence::Winner(ref board) => {
                    // Game could end here. Give the best score and move to the next branch.
                    let score = Score::new(1_f64, 0);
                    choices[route].set_score(score);
                    //drop(choices);
                    direction.scores.insert(curr_player, score);
                    direction.route += 1;
                    //drop(direction);
                    continue;
                },
                Consequence::Continue(ref board) | Consequence::TurnOver(ref board) => {
                    
                },
            }
        } else {
            
        }
    }
}

#[cfg(test)]
mod test {
    
    use crate::hexagon::{Cube, Grid};
    use crate::game;
    use super::*;

    #[test]
    fn three_quarters_two_player() {
        let board = game::canned_2x2_start01();
        let scores = score_board01(&board);
        let mut players = board.players().playing();
        let player2 = players.pop().unwrap();
        let player1 = players.pop().unwrap();
        
        assert!(scores.len() == 2);
        assert!(*scores.get(&player1).unwrap() == 0.25_f64);
        assert!(*scores.get(&player2).unwrap() == 0.75_f64);
    }

    #[test]
    fn insta_win() {
        let mut tree: Tree = game::canned_1x1_start().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 1_f64);
        assert!(*score.distance() == 0);
    }
}
