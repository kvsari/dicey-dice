//! Player details handling. To simplify other code from having to calculate who the
//! the next player is etc.
//!
//! This exists in its own module as it contains lots of code and doesn't depend on
//! anything else within this project.
use std::{fmt, mem};

use rand::Rng;
use rand::distributions::Distribution;
use derive_getters::Getters;

const MAX_PLAYERS: usize = 8;

/// Describes a player.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Getters)]
pub struct Player {
    number: usize,
    display: char,
}

impl Player {
    pub fn new(number: usize, display: char) -> Self {
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

/// Player management rolled into one struct. Keeps track of the current player and
/// emits the next player. There is an upper limit of `MAX_PLAYERS` players.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Players {
    players: usize,
    current: usize,
    count: usize,
    playing: [Option<Player>; MAX_PLAYERS],
    out: [Option<Player>; MAX_PLAYERS],
}

impl Players {
    /// If `players` is larger than `MAX_PLAYERS`, will truncate to `MAX_PLAYERS`. If
    /// `players` is less than 2, will use a minimum of 2.
    pub fn new(players: usize) -> Self {
        let current = 0;
        let mut playing = [None; MAX_PLAYERS];
        let players = if players > MAX_PLAYERS {
            MAX_PLAYERS
        } else if players < 2 {
            2
        } else {
            players
        };

        playing
            .iter_mut()
            .enumerate()
            .for_each(|(index, slot)| {
                let character: char = ((65 + index) as u8).into();
                let player = Player::new(index + 1, character);
                let mut n_state = Some(player);
                mem::swap(slot, &mut n_state);
            });

        Players {
            players,
            current,
            count: players,
            playing,            
            out: [None; MAX_PLAYERS],
        }
    }

    pub fn player_count(&self) -> usize {
        self.count
    }

    pub fn current(&self) -> Player {
        self.playing[self.current].unwrap()
    }

    /// Create a copy of self with the current player index incremented.
    pub fn next(&self) -> Self {
        let mut new_self = self.to_owned();
        new_self.current += 1;
        if new_self.current >= self.count {
            new_self.current = 0;
        }
        new_self
    }

    /// Returns a new `Players` struct with the current player moved into the `out` slot.
    /// The `count` will be reduced by one and the new current player will be moved to the
    /// next one. It is not possible to remove the last player as subsequent calls will just
    /// return a copy of `self`.
    pub fn remove_current(&self) -> Self {
        let mut new_self = self.to_owned();
        
        if new_self.count == 1 {
            return new_self;
        }

        new_self.count -= 1;
        let mut player = new_self.playing[new_self.current].take();
        assert!(player.is_some());
        mem::swap(&mut new_self.out[new_self.current], &mut player);

        // shuffle down by one all after current.
        for i in (new_self.current + 1)..MAX_PLAYERS {
            if new_self.playing[i].is_some() {
                let mut shuffle = new_self.playing[i].take();
                mem::swap(&mut new_self.playing[i - 1], &mut shuffle);
            }
        }

        if new_self.current >= new_self.count {
            new_self.current = 0;
        }

        new_self
    }
}

impl Distribution<Player> for Players {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
        self.playing[rng.gen_range(0, self.count)].unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initialize() {
        let players = Players::new(1);
        assert!(players.player_count() == 2);

        let players = Players::new(MAX_PLAYERS * 10);
        assert!(players.player_count() == MAX_PLAYERS);
    }

    #[test]
    fn next_player() {
        let players = Players::new(4);
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let player3 = Player::new(3, 'C');
        let player4 = Player::new(4, 'D');

        assert!(player1 == players.current());
        let players = players.next();
        assert!(player2 == players.current());
        let players = players.next();
        assert!(player3 == players.current());
        let players = players.next();
        assert!(player4 == players.current());
        let players = players.next();
        assert!(player1 == players.current());
    }

    #[test]
    fn remove_players() {
        let players = Players::new(4);
        let player1 = Player::new(1, 'A');
        let player2 = Player::new(2, 'B');
        let player3 = Player::new(3, 'C');
        let player4 = Player::new(4, 'D');

        assert!(player1 == players.current());
        let players = players.remove_current();
        assert!(players.player_count() == 3);
        assert!(player2 == players.current());
        let players = players.next();
        assert!(player3 == players.current());
        let players = players.remove_current();
        assert!(players.player_count() == 2);
        assert!(player4 == players.current());
        let players = players.next();
        assert!(player2 == players.current());
    }
}
