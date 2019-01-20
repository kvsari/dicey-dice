//! Console operations for playing the game on the command line. Aiming for bare minimum simplicity
//! here and not some fancy term driven app. The console is to verify that the engine works. The
//! intended UI will be something else.
use crate::game::tree::Tree;

pub fn session(tree: &mut Tree) {
    println!("Starting game!");

    while tree.game_on() {
        println!("{}", tree.current_traversal())
    }
}

pub fn handle_turn(tree: &Tree) {
    // 1. Print the state of the board.
    println!("{}", tree.current_traversal());

    // 2. Get all the options the player has.
    let available_moves = tree.available_moves();

    // 3. Print it out as a nice list.
    println!("Movement options. Or 'q' to quit.");
    available_moves
        .iter()
        .enumerate()
        .for_each(|(num, mv)| {
            println!("{}. {}", num + 1, mv);
        });

    // 4. Get player input with 'q' for quitting.

    // 5. Validate. Loop if necessary.

    // 6. Return the move that was chosen.
}
