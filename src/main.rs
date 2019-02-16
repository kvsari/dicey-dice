//! Console entrypoint
use std::error;

extern crate dicey_dice as lib;

use lib::{session, game, console};

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    println!("Dicey Dice starting...");

    //let players = game::Players::new(2);
    //let start = game::generate_random_board(4, 4, players);
    //let start = game::canned_2x2_start01();
    let start = game::canned_3x1_start01();

    println!("Using this board:\n{}", &start);
    
    let session = session::Setup::new()
        .set_board(start)
        .session()?;

    console::play_session(session);

    Ok(())
}
