//! Primitive AI that works on scoring moves in advance and chooses the highest scoring one
//! during play.
use std::collections::HashMap;


use crate::game::{Board, Players, Player};

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

/*
/// Traverse the tree from the start position to the end, then go backwards assigning a
/// multiple of the ending boards scores to each tile position that leads to them favouring
/// scoring that leads to winning boards for each player.
pub fn score_tree(
*/
