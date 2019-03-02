//! Primitive AI that works on scoring moves in advance and chooses the highest scoring one
//! during play.
use std::collections::HashMap;

use crate::game::{Board, Players, Player, Tree};

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

/// Traverse the tree creating a score for each move for the current player. The score will indicate a
/// path to a winning position.
pub fn score_tree(tree: &mut Tree) {
}

/*
/// Traverse the tree from the start position to the end, then go backwards assigning a
/// multiple of the ending boards scores to each tile position that leads to them favouring
/// scoring that leads to winning boards for each player.
pub fn score_tree(
*/

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
}
