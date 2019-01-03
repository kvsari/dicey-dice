//! Console entrypoint

extern crate dicey_dice_lib as lib;

use rand::prelude::*;

use lib::{hexagon, game};

/// TODO: Move this into a `game` module or something.
fn generate_random_2x2_board_game() -> hexagon::grid::Rectangular<game::hold::Hold> {
    let mut rng = thread_rng();    
    let grid = hexagon::grid::Rectangular::generate(2, 2, game::hold::Hold::default());
    
    grid.fork(move |_| {
        let player_code = rng.gen_range(1, 3);
        let player_dice = rng.gen_range(1, 6);
        game::hold::Hold::new(player_code, player_dice)
    })
}

fn main() {
    println!("Dicey Dice starting...");

    //let grid = grid::Rectangular::generate(2, 2, "A4");
    let grid = generate_random_2x2_board_game();
    println!("{}", &grid);
}
