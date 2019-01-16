//! Player details handling. To simplify other code from having to calculate who the
//! the next player is etc.
use std::{fmt, mem};

use derive_getters::Getters;
use rand::Rng;
use rand::distributions::Distribution;

const MAX_PLAYERS: usize = 8;

/// Describes a player.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Getters)]
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

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.display)
    }
}

#[derive(Debug, Copy, Clone)]
enum State {
    Present(Player),
    Removed(Player),
    Empty,
}

impl State {
    fn new(player: Player) -> Self {
        State::Present(player)
    }

    fn empty() -> Self {
        State::Empty
    }

    fn remove(&self) -> Self {
        match self {
            State::Present(p) => State::Removed(p),
            State::Removed(p) => State::Removed(p),
            State::Empty => State::Empty,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::empty()
    }
}

/// Player management rolled into one struct. Keeps track of the current player and
/// emits the next player. There is an upper limit of `MAX_PLAYERS` players.
#[derive(Debug, Copy, Clone)]
pub struct Players {
    players: usize,
    current: usize,
    playing: usize,
    roster: [State; MAX_PLAYERS],
}

impl Players {
    /// If `players` is larger than `MAX_PLAYERS`, will truncate to `MAX_PLAYERS`. If
    /// `players` is less than 2, will use a minimum of 2.
    pub fn new(players: usize) -> Self {
        let players = if players > MAX_PLAYERS {
            MAX_PLAYERS
        } else if players < 2 {
            2
        } else {
            players
        };

        let playing = players;
        let current = 0;

        let mut roster = [State::default(); MAX_PLAYERS];

        roster
            .iter_mut()
            .enumerate()
            .for_each(|(index, state)| {
                let character: char = ((65 + index) as u8).into();
                let player = Player::new(index, character);
                let mut n_state = State::new(player);
                mem::swap(state, &mut n_state);
            });

        Players {
            players, current, playing, roster
        }
    }

    pub fn current(&self) -> Player {
        self.players[self.current]
    }

    /*
    pub fn next(&self) -> Player {
        self.current += 1;
        if self.current >= self.count {
            self.current = 0;
        }
        self.current()
    }
    */
}

impl Distribution<Player> for Players {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
        self.players[rng.gen_range(0, self.count)]
    }
}
