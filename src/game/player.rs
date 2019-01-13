//! Player details handling. To simplify other code from having to calculate who the
//! the next player is etc.
use std::mem;

use derive_getters::Getters;

const MAX_PLAYERS: usize = 8;

/// Contains the current player.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Getters)]
pub struct Player {
    number: usize,
    display: char,
}

impl Player {
    fn new(number: usize, display: char) -> Self {
        Player {
            number,
            display,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            number: MAX_PLAYERS + 1,
            display: '~',
        }
    }
}

/// Player management rolled into one struct. Keeps track of the current player and
/// emits the next player. There is an upper limit of `MAX_PLAYERS` players.
#[derive(Debug, Copy, Clone)]
pub struct Players {
    count: usize,
    current: usize,
    players: [Player; MAX_PLAYERS],
}

impl Players {
    /// If `players` is larger then `MAX_PLAYERS`, will truncate to `MAX_PLAYERS`. If
    /// `players` is less than 2, will use a minimum of 2.
    pub fn new(players: usize) -> Self {
        let count = if players > MAX_PLAYERS {
            MAX_PLAYERS
        } else if players < 2 {
            2
        } else {
            players
        };

        let current = 0;

        let mut players = [Player::default(); MAX_PLAYERS];

        players
            .iter_mut()
            .enumerate()
            .for_each(|(index, player)| {
                let character: char = ((65 + index) as u8).into();
                let mut n_player = Player::new(index, character);
                mem::swap(player, &mut n_player);
            });

        Players {
            count, current, players
        }
    }
}
 
//impl Players {
//
//}

/*
pub struct Builder {
    players: Vec<char>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, character: char) -> 

    pub fn build(&self) -> Result<Players, &'static str> {
        if self.players.len() < 2 {
            return Err("Need at least two players.");
        }

        if self.players.len() > MAX_PLAYERS {
            return Err("Too many players.");
        }

        let mut players = [Player::default(); MAX_PLAYERS];
        self.players
            .iter()
            .enumerate()
            .for_each(|(count, player)| {
                players[count] = Player::new(count, *player);
            });

        Ok(Players {
            count: self.players.len(),
            current: 0,
            players,
        })
    }
}
*/
