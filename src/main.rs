//! Console entrypoint
use std::error;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::NonZeroU8;

extern crate dicey_dice as lib;

use lib::{session, game, console};

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    println!("Dicey Dice starting...");

    let players = game::Players::new(2);
    let start = game::generate_random_board(8, 8, players);
    
    //let start = game::canned_2x2_start04();
    //let start = game::canned_3x1_start05();
    //let start = game::canned_3x2_start02();
    let start = game::canned_3x3_start01();

    println!("Using this board:\n{}", &start);

    let ai_player_a = game::Player::new(1, 'A');
    let ai_player_b = game::Player::new(2, 'B');
    let ai_player_c = game::Player::new(3, 'C');
    let ai_player_d = game::Player::new(4, 'D');
    let ai_player_e = game::Player::new(5, 'E');
    let ai_player_f = game::Player::new(6, 'F');
    let ai_players: HashSet<game::Player> = HashSet::from_iter(
        vec![
            ai_player_a,
            ai_player_b,
            //ai_player_c,
            //ai_player_d,
            //ai_player_e,
            //ai_player_f,
        ].into_iter()
    );
    
    let session = session::Setup::new()
        .set_board(start)
        //.set_move_limit(NonZeroU8::new(2).unwrap())
        .session()?;

    console::play_session(session);
    //console::play_session_with_ai(session, ai_players, 4);

    Ok(())
}
