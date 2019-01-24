//! Console entrypoint

extern crate dicey_dice_lib as lib;

use lib::game::{self, Tree};

fn main() {
    println!("Dicey Dice starting...");

    //let players = game::Players::new(2);
    //let start = game::generate_random_board(2, 2, players);
    let start = game::canned_2x2_start01();
    
    let tree: Tree = start.clone().into();

    //lib::console::handle_player_turn_input(&tree, &start);

    let _traversal = lib::console::session(&tree);
}
