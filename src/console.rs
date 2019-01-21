//! Console operations for playing the game on the command line. Aiming for bare minimum
//! simplicity here and not some fancy term driven app. The console is to verify that the
//! engine works. The intended UI will be something else.
use std::io;

use crate::game::tree::{Tree, Move, BoardState};

pub fn session(tree: &mut Tree) {
    println!("Starting game!");

    while tree.game_on() {
        if let Some(choice) = handle_player_turn_input(&tree) {
            assert!(tree.choose(choice));
        } else {
            println!("No winner.");
            return;
        }
    }

    let winner = tree.current_traversal().players().current();
    println!("Winner is {}", &winner);
}

pub fn handle_player_turn_input(tree: &Tree) -> Option<usize> {
    // 1. Print the state of the board.
    println!("{}", tree.current_traversal());

    // 2. Get all the options the player has.
    let available_moves = tree.available_moves();
    let move_count = available_moves.len();

    // 3. Print it out as a nice list.
    println!("Movement options. Or 0 (Zero) to quit.");
    print_moves_from_nexts(&available_moves);

    // 4. Get player input with 'q' for quitting.
    let mut selection = String::new();
    let choice: usize = loop {
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to readline.");
        println!("You selected: {}", &selection);
        
        // 5. Validate.
        match selection.trim().parse() {
            Ok(num) => {
                if num <= move_count {
                    break num;
                } else {
                    println!("Number is too large. Choose from 0 to {}", &move_count);
                }
            },
            Err(e) => println!("Invalid choice: {}. Try again (or 0 to quit).", &e),
        }
        selection.clear();
    };

    // 6. Return the move that was chosen.
    if choice == 0 {
        None
    } else {
        Some(choice - 1)
    }
}

pub fn print_moves_from_nexts(nexts: &[Move]) {
    nexts
        .iter()
        .enumerate()
        .for_each(|(num, mv)| {
            println!("{}. {}", num + 1, mv);
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
