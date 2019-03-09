//! Tree generation functions.
use std::collections::HashMap;

use super::model::*;
use super::rules::choices_from_board_only_pass_at_end;

pub fn build_tree(root: Board) -> Tree {
    let states = calculate_all_consequences(root.clone());
    Tree::new(root, states)
}

/// Function will build all boardstates from `start`  inserting them into the `states` map.
/// If the boardstate already exists will skip that boardstate. This function has no
/// horizon so it won't stop generating until the stack is empty.
pub fn calculate_all_consequences(start: Board) -> HashMap<Board, Vec<Choice>> {
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

#[cfg(test)]
mod test {
    use crate::game::*;
    use super::*;
    
    #[test]
    fn breadth_first_on_canned_2x1_start01() {
        let board = canned_2x1_start01();
        let (states, _stats) = breadth_first_calc_consequences(board.clone());
        assert!(states.len() == 3);
        assert!(states.contains_key(&board));
    }

    #[test]
    fn breadth_first_on_canned_2x2_start01() {
        let board = canned_2x2_start01();
        let (states, _stats) = breadth_first_calc_consequences(board.clone());
        assert!(states.len() == 4);
        assert!(states.contains_key(&board));
    }

    #[test]
    fn consequences_3x1_2player() {
        let board = canned_3x1_start01();
        let consequences = calculate_all_consequences(board.clone());

        assert!(consequences.len() == 8);
    }

    #[test]
    fn consequences_3x1_3player() {
        let board = canned_3x1_start05();
        let consequences = calculate_all_consequences(board.clone());

        assert!(consequences.len() == 9);
    }
}
