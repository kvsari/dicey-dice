//! Console entrypoint

extern crate dicey_dice_lib as lib;

use lib::game;

fn main() {
    println!("Dicey Dice starting...");

    let grid = game::generate_random_2x2_board_game();
    println!("{}", &grid);
}
