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
    scores: Option<HashMap<Player, f64>>,
}

impl Direction {
    pub fn new(board: Board, route: usize) -> Self {
        Direction { board, route, scores: None }
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

    // TODO: Finish me!
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
