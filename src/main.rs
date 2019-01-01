//! Console entrypoint

use rand::prelude::*;

mod hold;
mod grid;
mod coordinate;
mod errors;

/// TODO: Move this intoo a `game` module or something.
fn generate_random_2x2_board_game() -> grid::Rectangular<hold::Hold> {
    let mut rng = thread_rng();    
    let grid = grid::Rectangular::generate(2, 2, hold::Hold::default());
    
    grid.fork(move |_| {
        let player_code = rng.gen_range(1, 3);
        let player_dice = rng.gen_range(1, 6);
        hold::Hold::new(player_code, player_dice)
    })
}

fn main() {
    println!("Dicey Dice starting...");

    //let grid = grid::Rectangular::generate(2, 2, "A4");
    let grid = generate_random_2x2_board_game();
    println!("{}", &grid);
}
