//! Handle a game.
use crate::game::{Board, Players};

/// Setup a game session. Can set the number of players and the board size and to use
/// canned boards (feed it a starting position. The board can only be rectangular.
pub struct Setup {
    players: Players,
    rows: u32,
    columns: u32,
}

impl Setup {
    pub fn new(players: Players, rows: u32, columns: u32) -> Self {
        Setup { players, rows, columns }
    }
    
    pub fn set_players(&mut self, players: usize) -> &mut Self {
        self.players = Players::new(players);
        self
    }
}

impl Default for Setup {
    fn default() -> Self {
        Setup::new(Players::new(2), 2, 2)
    }    
}

pub struct Session {
    players: Players,
    traversal: Vec<Board>,
}
