//! Territorial hold. Every tile on the hex grid is a territorial holding of a player.
use std::fmt;

use derive_getters::Getters;

/// A territorial hold on a particular tile.
#[derive(Debug, Copy, Clone, Getters)]
pub struct Hold {
    owner: u32,

    /// We're assuming D6's here.
    dice: u32,
}

impl Hold {
    pub fn new(owner: u32, dice: u32) -> Hold {
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
