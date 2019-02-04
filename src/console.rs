//! Console operations for playing the game on the command line. Aiming for bare minimum
//! simplicity here and not some fancy term driven app. The console is to verify that the
//! engine works. The intended UI will be something else.
use std::io;

use crate::game::model::{Tree, Choice, Board, Consequence};
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
            Progression::PlayOn => (),
            Progression::GameOverWinner(player) => {
                println!("Game Over\nWinner is {}", &player);
                break;
            },
            Progression::GameOverStalemate(_players) => {
                // We won't bother printing the players yet. We have no code to detect
                // stalemates.
                println!("Game Over\nSTATELMATE");
                break;
            },
        }

        // 3. Get all the options the current player has.
        let curr_player = state.board().players().current().to_owned();
        let available_choices = state.choices();

        if let Some(index) = handle_player_turn_input(available_choices.as_slice()) {
            session.advance(&available_choices[index]).unwrap();
        } else {
            println!("Quitting game. No Winner.");
            break;
        }
    }
}

/// Plays a session of the game and returns the entire traversal.
pub fn session(tree: &Tree) -> Vec<Board> {
    println!("Starting game!");

    let mut traversal: Vec<Board> = vec![tree.start().clone()];

    loop {
        // 1. Print the state of the board.
        let board = traversal.last().unwrap().to_owned();
        println!("{}", &board);

        // 2. Get all the options the current player has.
        let curr_player = board.players().current().to_owned();
        let available_choices = tree.fetch_choices(&board).unwrap();

        // 3. If there is only one option left, it's a pass. Therefore we check if it's a
        // win/lose or turn over consequence.
        if available_choices.len() == 1 {
            match available_choices[0].consequence() {
                Consequence::TurnOver(next_board) => {
                    println!(
                        "Player {} can only pass. Moving to next player.",
                        &curr_player,
                    );
                    traversal.push(next_board.to_owned());
                    continue;
                },
                Consequence::GameOver(next_board) => {
                    println!(
                        "Player {} has been knocked out! Moving to next player.",
                        &curr_player,
                    );
                    traversal.push(next_board.to_owned());
                    continue;
                },
                Consequence::Winner(_) => {
                    println!("Player {} has won!", &curr_player);
                    break;
                },
                _ => panic!("Invalid consequence. Supposed to be a passing move."),
            }
        }

        // 4. Otherwise we need to give our player their right to exercise choices.
        if let Some(choice) = handle_player_turn_input(&available_choices) {
            traversal.push(available_choices[choice].consequence().board().to_owned());
        } else {
            println!("Player has opted to quit ending game. No winner determined.");
            break;
        }
    }

    traversal
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

/*
pub fn print_game_ended(boardstate: &BoardState) {
    let players = boardstate.players();

    if players.player_count() == 1 {
        println!("Player {} wins!", &players.current());
    } else {
        println!("No winner.");
    }
}
*/
