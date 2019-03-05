//! Primitive AI that works on scoring moves in advance and chooses the highest scoring one
//! during play.
use std::collections::HashMap;

use crate::game::{Board, Player, Tree, Consequence, Score};

/// One point/step on the traversal of the game tree.
#[derive(Debug, Clone)]
struct Direction {
    /// Index into the tree and also the current player.
    board: Board,

    /// The choice taken.
    route: usize,

    /// On rewinding, we store the scores.
    scores: HashMap<Player, Score>,
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
    let direction = Direction::new(board, 0);
    traversal.push(direction);

    while let Some(ref mut direction) = traversal.last_mut() {
        let choices = tree.mut_fetch_choices_unchecked(&direction.board);
        let route = direction.route;
        let player = direction.board.players().current();
        if choices.len() > route {
            let consequence = choices[route].consequence().to_owned();
            match consequence {
                Consequence::Stalemate(board) => {
                    // Game could end here. It's not an ideal end.
                    let scores = score_board01(&board);
                    let score = Score::new(*scores.get(&player).unwrap(), 0);
                    choices[route].set_score(score);
                    scores
                        .into_iter()
                        .for_each(|(player, dest)| {
                            let new_score = Score::new(dest, 0);
                            if let Some(existing_score) = direction.scores.get(&player) {
                                if *existing_score < new_score {
                                    direction.scores.insert(player, new_score);
                                }
                            } else {
                                direction.scores.insert(player, new_score);
                            }
                        });
                    direction.route += 1;
                    continue;
                },
                Consequence::GameOver(board) => {
                    // It is game over for the current player. But the game may continue.
                    let score = Score::new(0_f64, 0);
                    choices[route].set_score(score);
                    if let Some(existing_score) = direction.scores.get(&player) {
                        if *existing_score < score {
                            direction.scores.insert(player, score);
                        }
                    } else {
                        direction.scores.insert(player, score);
                    }
                    drop(direction);
                    traversal.push(Direction::new(board, 0));
                    continue;
                },
                Consequence::Winner(_) => {
                    // Game could end here. Give the best score and move to the next branch.
                    println!("Game WON!");
                    let score = Score::new(1_f64, 0);
                    choices[route].set_score(score);
                    direction.scores.insert(player, score);
                    direction.route += 1;
                    continue;
                },
                Consequence::Continue(board) | Consequence::TurnOver(board) => {
                    println!("Traversing!");
                    drop(direction);
                    traversal.push(Direction::new(board, 0));
                    continue;
                },
            }
        } else {            
            drop(choices);
            drop(direction);
            let direction = traversal.pop().unwrap();
            if let Some(ref mut preceding) = traversal.last_mut() {
                println!("Rewinding!");
                // Back propagate each score only if it is better.
                direction.scores
                    .into_iter()
                    .map(|(player, score)| (player, score.increment_distance()))
                    .for_each(|(player, new_score)| {
                        if let Some(p_score) = preceding.scores.get(&player) {
                            if new_score > *p_score {
                                drop(p_score);
                                preceding.scores.insert(player, new_score);
                            }
                        } else {
                            preceding.scores.insert(player, new_score);
                        }
                    });

                // We fetch the player at that stage and their score.
                let player = preceding.board.players().current();
                let score = *preceding.scores
                    .entry(player)
                    .or_insert_with(|| Score::new(0_f64, 0));
                
                // Apply the score to the previous choice if there is one better for the
                // player at that stage.
                let choices = tree.mut_fetch_choices_unchecked(&preceding.board);
                if let Some(existing_score) = *choices[preceding.route].score() {
                    if score > existing_score {
                        choices[preceding.route].set_score(score);
                    }
                } else {
                    choices[preceding.route].set_score(score);
                }

                // Increment the route to explore the next branch (if any).
                preceding.route += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
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
    fn insta_win_1x1() {
        let mut tree: Tree = game::canned_1x1_start().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 1_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn insta_win_2x1() {
        let mut tree: Tree = game::canned_2x1_start03().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 1_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn stalemate_2x1() {
        let mut tree: Tree = game::canned_2x1_start02().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 0.5_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn game_2x1() {
        let mut tree: Tree = game::canned_2x1_start01().into();
        score_tree(&mut tree);

        // First move
        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 0));

        // Second move
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 1));

        // Last move
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 0));
    }

    #[test]
    fn insta_win_3x1() {
        let mut tree: Tree = game::canned_3x1_start02().into();
        score_tree(&mut tree);

        // There are actually two moves as player 'B' is the winner. Player 'A' has to
        // game over first.
        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 0_f64);
        assert!(*score.distance() == 0);

        // Second move.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 0));
    }

    #[test]
    fn stalemate_3x1() {
        let mut tree: Tree = game::canned_3x1_start03().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() >= 0.6_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn game_3x1() {
        let mut tree: Tree = game::canned_3x1_start01().into();
        score_tree(&mut tree);

        // Player 'B' is the eventual winner. But player 'A' needs to pass first.
        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 4));

        // Second move
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 6));

        // Third move (passing)
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 5));

        // Fourth move. Player 'A' has their last attack.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 1));

        // Fifth move. Player 'A' has just as well lost. They never get a move again even
        // though they're still in the game.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 0));

        // Sixth move. Back to player 'B'.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 2));

        // Seventh move.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 1));

        // Final move. Player 'B' has won.
        let next_board = choices[0].consequence().board().to_owned();
        let choices = tree.fetch_choices(&next_board).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 0));
    }

    #[test]
    fn stalemate_3x1_v2() {
        let mut tree: Tree = game::canned_3x1_start04().into();
        score_tree(&mut tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() >= 0.3_f64);
        assert!(*score.distance() == 0);
    }
}
