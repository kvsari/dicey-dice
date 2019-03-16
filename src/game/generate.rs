//! Tree generation functions.
use std::collections::HashMap;

use super::model::*;
use super::rules::choices_from_board_only_pass_at_end;

/// Attemps construction of the entire tree. Can choke on 3x3 boards and will definitiely
/// OOM on 4x4 boards and above.
pub fn build_tree(root: Board, move_limit: u8) -> Tree {
    let states = calculate_all_consequences(root.clone(), move_limit);
    Tree::new(root, states)
}

/// Like above using brute force calculation to evaluate all board positions. But will stop
/// at the depth indicated by `horizon`.
pub fn start_tree_horizon_limited(
    root: Board, horizon: usize, move_limit: u8
) -> Tree {
    let states = calculate_consequences(root.clone(), horizon, move_limit);
    Tree::new(root, states)
}

/// Like above using brute force calculation to evaluate all board positions. But will stop
/// when the specified number of boards, the budget, has been inserted. This will leave
/// the last depth layer incomplete in almost all cases.
///
/// **NOTE**, the first layer will always be computed otherwise valid choices from the
/// start will be denied to the player. This is only an issue on insane 100x100 boards.
pub fn start_tree_insert_budgeted(
    root: Board, board_budget: usize, move_limit: u8,
) -> Tree {
    let states = calculate_consequences_insert_limited(
        root.clone(), board_budget, move_limit,
    );
    Tree::new(root, states)
}

/// Adds to the sent tree. If the `Board` is not within the tree, it is returned as Err.
pub fn grow_tree_horizon_limited(
    from: Board, horizon: usize, tree: &mut Tree, move_limit: u8,
) -> Result<(), Board> {
    let _ = tree.fetch_choices(&from).ok_or_else(|| from.clone());

    // Fairly wasteful as many positions already calculated will be re-calculated.
    let new_states = calculate_consequences(from, horizon, move_limit);
    tree.append(new_states);
    
    Ok(())
}

/// Function will build all boardstates from `start`  inserting them into the `states` map.
/// If the boardstate already exists will skip that boardstate. This function has no
/// horizon so it won't stop generating until the stack is empty.
pub fn calculate_all_consequences(
    start: Board, move_limit: u8
) -> HashMap<Board, Vec<Choice>> {
    let (tree, stats) = breadth_first_calc_consequences(start, move_limit);

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

pub fn calculate_consequences(
    from: Board, horizon: usize, move_limit: u8,
) -> HashMap<Board, Vec<Choice>> {
    let (tree, stats) = bounded_breadth_first_calc_consequences(from, horizon, move_limit);

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

pub fn calculate_consequences_insert_limited(
    from: Board, board_budget: usize, move_limit: u8,
) -> HashMap<Board, Vec<Choice>> {
    let (tree, stats) = insert_budgeted_breadth_first_calc_consequences(
        from, board_budget, move_limit,
    );

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
    start: Board, move_limit: u8,
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
                let choices = choices_from_board_only_pass_at_end(&board, move_limit);
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

/// Brute force the tree with a horizon limit. Only calculate to the depth specified.
fn bounded_breadth_first_calc_consequences(
    start: Board, horizon: usize, move_limit: u8,
) -> (HashMap<Board, Vec<Choice>>, Vec<LayerStats>) {
    let mut states: HashMap<Board, Vec<Choice>> = HashMap::new();
    let mut current_layer: Option<Vec<Board>> = Some(vec![start]);
    let mut layer_count: usize = 0;
    let mut layer_stats: Vec<LayerStats> = Vec::new();
    
    for _depth in 0..horizon {
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
                let choices = choices_from_board_only_pass_at_end(&board, move_limit);
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

/// Brute force the tree with a board insert limit. Only calculate to the boards specified.
/// Will not cancel a partially computed depth layer.
fn insert_budgeted_breadth_first_calc_consequences(
    start: Board, boards: usize, move_limit: u8,
) -> (HashMap<Board, Vec<Choice>>, Vec<LayerStats>) {
    let mut spent: usize = 0;
    let mut states: HashMap<Board, Vec<Choice>> = HashMap::new();
    let mut current_layer: Option<Vec<Board>> = Some(vec![start]);
    let mut layer_count: usize = 0;
    let mut layer_stats: Vec<LayerStats> = Vec::new();
    
    while spent < boards {
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
                let choices = choices_from_board_only_pass_at_end(&board, move_limit);
                next_layer.extend(
                    choices
                        .iter()
                        .map(|choice| choice.consequence().board().to_owned())
                );
                states.insert(board, choices);

                // Prepare more stats.
                board_inserts += 1;

                // We start budgeting from the second layer. This way the start always has
                // all valid moves calculated.
                if layer_count > 1 {
                    spent += 1;
                }
            }
            if spent > boards {
                break;
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
        let (states, _stats) = breadth_first_calc_consequences(board.clone(), 10);
        assert!(states.len() == 3);
        assert!(states.contains_key(&board));
    }

    #[test]
    fn breadth_first_on_canned_2x2_start01() {
        let board = canned_2x2_start01();
        let (states, _stats) = breadth_first_calc_consequences(board.clone(), 10);
        assert!(states.len() == 4);
        assert!(states.contains_key(&board));
    }

    #[test]
    fn consequences_3x1_2player() {
        let board = canned_3x1_start01();
        let consequences = calculate_all_consequences(board.clone(), 10);

        assert!(consequences.len() == 2);
    }

    #[test]
    fn consequences_3x1_3player() {
        let board = canned_3x1_start05();
        let consequences = calculate_all_consequences(board.clone(), 20);

        assert!(consequences.len() == 14);
    }
}
