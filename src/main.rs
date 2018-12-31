//! Console entrypoint

mod hold;
mod grid;
mod coordinate;
mod errors;

fn main() {
    println!("Dicey Dice starting...");

    let grid = grid::Rectangular::generate(2, 2, "A4");
    println!("{}", &grid);
}
