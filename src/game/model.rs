//! Game data structures
use std::collections::HashMap;
use std::fmt;

use derive_getters::Getters;

use crate::hexagon::{Cube, Grid};
use super::{Player, Players};
use super::rules::calculate_all_consequences;

pub type FromHex = Cube;
pub type ToHex = Cube;

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

impl Default for Hold {
    fn default() -> Self {
        Hold::new(Player::default(), 0)
    }
}

/// The full state of the game. Represents an iteration of play.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Getters)]
pub struct Board {
    players: Players,
    grid: Grid<Hold>,
    captured_dice: u32,
}

impl Board {
    pub fn new(players: Players, grid: Grid<Hold>, captured_dice: u32) -> Self {
        Board { players, grid, captured_dice }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Current Player: {}\nCaptured Dice: {}\nBoard =============\n{}",
            &self.players.current(),
            &self.captured_dice,
            &self.grid,
        )
    }
}

/// A legal player action that will advance the game state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Attack(FromHex, ToHex),
    Pass,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Attack(from, to) => write!(f, "Attack from {} into {}", from, to),
            Action::Pass => write!(f, "Pass turn."),
        }
    }
}

/// What follows from a `Move`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consequence {
    Continue(Board),
    TurnOver(Board),
    GameOver(Board),
    Winner(Board),
}

impl Consequence {
    pub fn board(&self) -> &Board {
        match self {
            Consequence::Continue(ref b) => b,
            Consequence::TurnOver(ref b) => b,
            Consequence::GameOver(ref b) => b,
            Consequence::Winner(ref b) => b
        }
    }
}

/// A `Choice` which that is an `Action` with its `Consequence`.
#[derive(Debug, Clone, Getters)]
pub struct Choice {
    action: Action,
    consequence: Consequence,
}

impl Choice {
    pub fn new(action: Action, consequence: Consequence) -> Self {
        Choice { action, consequence }
    }
}

/// The game tree. Contains all moves possible from the starting state.
#[derive(Debug, Clone, Getters)]
pub struct Tree {
    start: Board,
    states: HashMap<Board, Vec<Choice>>,
}

/// I just feel dirty doing `impl Tree { pub fn new(b: Board) -> Self ... ` for some reason.
/// Depending on the size of the board, this could take a long time or cause an OOM error.
impl From<Board> for Tree {
    fn from(b: Board) -> Self {
        Tree {
            start: b.clone(),
            states: calculate_all_consequences(b),
        }
    }
}
