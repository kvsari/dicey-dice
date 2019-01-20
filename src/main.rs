//! Console entrypoint

extern crate dicey_dice_lib as lib;

use lib::game;

fn main() {
    println!("Dicey Dice starting...");

    //let players = game::Players::new(2);
    //let starting_grid = game::generate_random_grid(2, 2, players);
    let start = game::canned_2x2_start01();
    
    let tree = game::tree::grow_entire_tree_from(start);

    //println!("RAW GAME TREE: {:?}", &tree);

    //println!("{}", tree.current_traversal());

    lib::console::handle_turn(&tree);
}
