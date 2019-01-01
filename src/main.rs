//! Console entrypoint

mod hold;
mod grid;
mod coordinate;
mod errors;

/// TODO: Move this intoo a `game` module or something.
fn generate_random_2x2_board_game() -> grid::Rectangular<hold::Hold> {
    let grid = grid::Rectangular::generate(2, 2, hold::Hold::default());    
    
    //grid.fork(
    grid
}

fn main() {
    println!("Dicey Dice starting...");

    //let grid = grid::Rectangular::generate(2, 2, "A4");
    //println!("{}", &grid);
}
