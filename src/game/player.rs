//! Player details handling. To simplify other code from having to calculate who the
//! the next player is etc.

/// Contains the current player and some details about them.
#[derive(Debug, Copy, Clone)]
pub struct Player {
    number: u8,
    display: Char,
}

/// Contains all players in a game.
pub struct Players {
    
}
 
