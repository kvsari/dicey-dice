//! Console entrypoint

extern crate dicey_dice_lib as lib;

use lib::game;

fn main() {
    println!("Dicey Dice starting...");

    let players = game::Players::new(2);
    let starting_grid = game::generate_random_grid(3, 3, players);

    let tree = game::tree::grow_entire_tree_from(starting_grid, players);

    //println!("RAW GAME TREE: {:?}", &tree);
}
