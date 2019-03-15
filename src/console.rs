//! Console operations for playing the game on the command line. Aiming for bare minimum
//! simplicity here and not some fancy term driven app. The console is to verify that the
//! engine works. The intended UI will be something else.
use std::io;
use std::collections::HashSet;

use crate::game::{Player, Choice, Score};
use crate::session::{Progression, Session};

pub fn play_session(mut session: Session) {
    println!("Starting game session!");

    loop {        
        // 1. Print the state of the board.
        // TODO: Print any progression too.
        let state = session.current_turn().to_owned();
        println!("{}", state.board());
        
        // 2. Check if we game on.
        match state.game() {
            Progression::PlayOn(outcome) => println!("{}", &outcome),
            Progression::GameOverWinner(player) => {
                println!("Game Over\nWinner is {}", &player);
                break;
            },
            Progression::GameOverStalemate(players) => {
                println!("Game Over\nSTATELMATE between players {:?}", &players);
                break;
            },
        }

        // 3. Get all the options the current player has.
        let available_choices = state.choices();

        if let Some(index) = handle_player_turn_input(available_choices.as_slice()) {
            session.advance(index).unwrap();
        } else {
            println!("Quitting game. No Winner.");
            break;
        }
    }
}

/// Passed in `Session` must have AI scoring enabled during setup.
pub fn play_session_with_ai(
    mut session: Session, ai_players: HashSet<Player>, compute_budget: usize,
) {
    println!("Starting game session with {} AI players.", &ai_players.len());

    loop {        
        // 1. Print the state of the board.
        // TODO: Print any progression too.
        let state = session.current_turn().to_owned();
        println!("{}", state.board());
        
        // 2. Check if we game on.
        match state.game() {
            Progression::PlayOn(outcome) => println!("{}", &outcome),
            Progression::GameOverWinner(player) => {
                println!("Game Over\nWinner is {}", &player);
                break;
            },
            Progression::GameOverStalemate(players) => {
                println!("Game Over\nSTATELMATE between players {:?}", &players);
                break;
            },
        }

        // 3. Get the current player.
        let curr_player = state.board().players().current().to_owned();
        let available_choices = state.choices();

        // 4. Check if the current player is an AI player.
        let choice = if ai_players.contains(&curr_player) {
            drop(available_choices);
            drop(state);
            //let state = session.score_with_insert_budget(compute_budget);
            let state = session.score_with_depth_horizon(compute_budget);
            handle_ai_turn(state.choices().as_slice())
        } else {
            handle_player_turn_input(available_choices.as_slice())
        };

        if let Some(index) = choice {
            session.advance(index).unwrap();
        } else {
            println!("Quitting game. No Winner.");
            break;
        }
    }
}

/// The board must be a valid key within the tree. Otherwise panic.
pub fn handle_player_turn_input(choices: &[Choice]) -> Option<usize> {
    let choice_count = choices.len();
    
    // 1. Print it out as a nice list.
    println!("Movement options. Or 0 (Zero) to quit.");
    print_actions_from_choices(&choices);

    // 2. Get player input with 'q' for quitting.
    let mut selection = String::new();
    let choice: usize = loop {
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to readline.");
        println!("You selected: {}", &selection);
        
        // 3. Validate.
        match selection.trim().parse() {
            Ok(num) => {
                if num <= choice_count {
                    break num;
                } else {
                    println!("Number is too large. Choose from 0 to {}", &choice_count);
                }
            },
            Err(e) => println!("Invalid choice: {}. Try again (or 0 to quit).", &e),
        }
        selection.clear();
    };

    // 4. Return the move that was chosen.
    if choice == 0 {
        None
    } else {
        Some(choice - 1)
    }
}

pub fn print_actions_from_choices(choices: &[Choice]) {
    choices
        .iter()
        .map(|c| c.action())
        .enumerate()
        .for_each(|(num, act)| {
            println!("{}. {}", num + 1, act);
        }); 
}

/// Rely on the choice scoring to move.
pub fn handle_ai_turn(choices: &[Choice]) -> Option<usize> {
    let (index, _) = choices
        .iter()
        .enumerate()
        .fold((0, Score::default()), |(index, best), (count, choice)| {
            let score = choice.score().unwrap();
            if score > best {
                (count, score)
            } else {
                (index, best)
            }
        });

    Some(index)
}
