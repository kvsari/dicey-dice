//! Console operations for playing the game on the command line.
use crate::game::tree::Tree;

pub fn session(tree: &mut Tree) {
    println!("Starting game!");

    while tree.game_on() {
        let curr_player = tree.current_traversal().players().current();
        println!("Current Player: {}", curr_player);
    }
}
