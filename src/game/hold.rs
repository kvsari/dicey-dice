//! Territorial hold. Every tile on the hex grid is a territorial holding of a player.
use std::{default, fmt};

use derive_getters::Getters;

use super::player::Player;

/// A territorial hold on a particular tile.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Getters)]
pub struct Hold {
    /// AKA the player.
    owner: Player,

    /// We're assuming D6's here.
    dice: u8,
}

impl Hold {
    pub fn new(owner: Player, dice: u8) -> Hold {
        Hold {
            owner, dice
        }
    }
}

impl fmt::Display for Hold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", &self.owner, &self.dice)
    }
}

impl default::Default for Hold {
    fn default() -> Self {
        Hold::new(Player::default(), 0)
    }
}
