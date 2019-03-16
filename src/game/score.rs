//! Primitive AI that works on scoring moves in advance and chooses the highest scoring one
//! during play.
use std::collections::HashMap;
use std::mem;

use super::{Board, Player, Tree, Consequence, Score, Holding};

/// Wipe all scoring from the tree.
pub fn clear_all_scoring(tree: &Tree) {
    clear(tree.root(), tree);
}

/// Like above but only starting from the specified board.
pub fn clear_scoring_from(from: &Board, tree: &Tree) {
    clear(from, tree);
}

fn clear(board: &Board, tree: &Tree) {
    let choices = match tree.fetch_choices(board) {
        Some(choices) => choices,
        None => return,
    };

    for choice in choices {
        if choice.score().is_none() {
            continue;
        }
        
        match choice.consequence() {
            Consequence::GameOver(ref board) |
            Consequence::Continue(ref board) |
            Consequence::TurnOver(ref board) => {
                clear(board, tree);
            },
            _ => (),
        }
        choice.clear_score();
    }
}

/// Score all the nodes moves in the tree. Return the number of moves scored.
pub fn score_tree(tree: &Tree) -> usize {
    let (touched, _) = score(tree.root(), tree);
    touched
}

/// Score a section of the tree starting from the supplied `Board`.
pub fn score_tree_from(from: &Board, tree: &Tree) -> usize {
    let (touched, _) = score(from, tree);
    touched
}

/// Look at a board and calculate a score from 0 to 1 for all the `Players`. It assumes
/// that the board has already been checked to not be a winning or losing board.
///
/// This will create a score by calculating the percentage of occupied tiles. No further
/// analysis is done.
fn score_board(board: &Board) -> HashMap<Player, Score> {
    let mut count: HashMap<Player, usize> = HashMap::new();
    let tiles = board.grid().len() as f64;
    
    board
        .grid()
        .iter()
        .for_each(|ht| {
            count.entry(ht.data().owner())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

    count
        .into_iter()
        .map(|(player, held)| {
            let held = held as f64;
            (player, Score::new(held / tiles, 0))
        })
        .collect()
}

fn score(board: &Board, tree: &Tree) -> (usize, HashMap<Player, Score>) {    
    let mut scores: HashMap<Player, Score> = HashMap::new();
    let player = board.players().current();
    let choices = match tree.fetch_choices(board) {
        Some(choices) => choices,
        None => {
            // The tree has been partially calculated and we've reached the end. Score the
            // board as it stands and return it.
            return (0, score_board(board))
        },
    };
    let mut sum = 0;
    for choice in choices {
        // Since we are using a hashmap as the underlying tree data storage, there's a
        // chance that cycles can appear. Thus, if this choice already has a score, we
        // skip it. Otherwise we'll loop unecessarily re-visiting already scored choices.
        if choice.score().is_some() {
            continue;
        }
        
        let consequence = choice.consequence();
        let (visited, sub_scores) = match consequence {
            Consequence::Stalemate(ref board) => {
                // Game could end here. It's not an ideal end.
                let sub_scores = score_board(&board);
                choice.set_score(*sub_scores.get(&player).unwrap());                
                return (1, sub_scores);
            },
            Consequence::Winner(_) => {
                // Game could end here. Give the best score and return.
                let win_score = Score::new(1_f64, 0);
                choice.set_score(win_score);
                let mut sub_scores: HashMap<Player, Score> = HashMap::with_capacity(1);
                sub_scores.insert(player, win_score);
                return (1, sub_scores);
            },
            Consequence::GameOver(ref board) => {
                // It is game over for the current player. But the game continues.
                let game_over_score = Score::new(0_f64, 0);
                let (v, mut sc) = score(board, tree);
                assert!(sc.insert(player, game_over_score).is_none());
                choice.set_score(game_over_score);
                (v, sc)
            },
            Consequence::Continue(ref board) | Consequence::TurnOver(ref board) => {
                let (v, mut sc) = score(board, tree);
                // A player that has lost may never get the chance to `GameOver` as the
                // game would end before their next turn. Thus their score is absent
                // which will cause a crash if this trunk node was their last play.
                sc.entry(player)
                    .and_modify(|s| {
                        choice.set_score(s.increment_distance());
                    })
                    .or_insert_with(|| {
                        let s = Score::new(0_f64, 0);
                        choice.set_score(s);
                        s
                    });
                (v, sc)
            },
        };
        
        // If we reached here, this choice was a trunk and not a leaf.        
        // Back propagate each sub score only if it is better.
        sub_scores
            .into_iter()
            .map(|(player, score)| (player, score.increment_distance()))
            .for_each(|(player, mut new_score)| {
                scores
                    .entry(player)
                    .and_modify(|mut current_score| {
                        if new_score > *current_score {
                            mem::swap(&mut new_score, &mut current_score);
                        }
                    })
                    .or_insert(new_score);
            });

        // Increase this insipid counter.
        sum += visited;
    }    

    (sum + 1, scores)
}

#[cfg(test)]
mod test {
    use crate::game;
    use super::super::build_tree;
    use super::*;

    #[test]
    fn three_quarters_two_player() {
        let board = game::canned_2x2_start01();
        let scores = score_board(&board);
        let mut players = board.players().playing();
        let player2 = players.pop().unwrap();
        let player1 = players.pop().unwrap();
        
        assert!(scores.len() == 2);
        assert!(*scores.get(&player1).unwrap().destination() == 0.25_f64);
        assert!(*scores.get(&player2).unwrap().destination() == 0.75_f64);
    }

    #[test]
    fn insta_win_1x1() {
        let tree = build_tree(game::canned_1x1_start(), 100);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 1_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn insta_win_2x1() {
        let tree = build_tree(game::canned_2x1_start03(), 100);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 1_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn stalemate_2x1() {
        let tree = build_tree(game::canned_2x1_start02(), 20);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() == 0.5_f64);
        assert!(*score.distance() == 0);
    }

    #[test]
    fn game_2x1() {
        let tree = build_tree(game::canned_2x1_start01(), 20);
        score_tree(&tree);

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
        let tree = build_tree(game::canned_3x1_start02(), 10);
        score_tree(&tree);

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
        let tree = build_tree(game::canned_3x1_start03(), 20);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() >= 0.6_f64);
        assert!(*score.distance() == 0);
    }

    /*
    #[test]
    fn game_3x1() {
        let tree = build_tree(game::canned_3x1_start01(), 100);
        score_tree(&tree);

        // Player 'B' is the eventual winner. But player 'A' needs to pass first.
        let choices = tree.fetch_choices(tree.root()).unwrap();
        dbg!(choices);
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 5));

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
        assert!(choices[0].score().unwrap() == Score::new(0_f64, 2)); // ?? Should be 1?

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
     */

    /// Redo of the test above due to rules changes allowing equal die amounts to fight
    /// because dice rolling has been introduced. Thus 'A' now wins very quickly.
    #[test]
    fn game_3x1() {
        let tree = build_tree(game::canned_3x1_start01(), 20);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        assert!(choices[0].score().unwrap() == Score::new(1_f64, 1));
    }

    #[test]
    fn stalemate_3x1_v2() {
        let tree = build_tree(game::canned_3x1_start04(), 100);
        score_tree(&tree);

        let choices = tree.fetch_choices(tree.root()).unwrap();
        assert!(choices.len() == 1);
        let score = choices[0].score().unwrap();
        assert!(*score.destination() >= 0.3_f64);
        assert!(*score.distance() == 0);
    }
}
