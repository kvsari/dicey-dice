//! Console entrypoint
use std::error;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate dicey_dice as lib;

use lib::{session, game, console};

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    println!("Dicey Dice starting...");

    //let players = game::Players::new(2);
    //let start = game::generate_random_board(2, 2, players);
    //let start = game::canned_2x2_start05();
    //let start = game::canned_3x1_start05();
    let start = game::canned_3x2_start02();
    //let start = game::canned_3x3_start03();

    println!("Using this board:\n{}", &start);

    let ai_player_b = game::Player::new(2, 'B');
    let ai_player_c = game::Player::new(3, 'C');
    let ai_players: HashSet<game::Player> = HashSet::from_iter(
        vec![ai_player_b, ai_player_c].into_iter()
    );
    
    let session = session::Setup::new()
        .set_board(start)
        .enable_ai_scoring()
        .session()?;

    //console::play_session(session);
    console::play_session_with_ai(session, ai_players);

    Ok(())
}
