//! Game data structures
use std::collections::HashMap;
use std::cell::Cell;
use std::{fmt, ops, cmp};

use derive_getters::Getters;

use crate::hexagon::{Cube, Grid};
use super::{Player, Players, player};

pub type FromHex = Cube;
pub type ToHex = Cube;
pub type Capturing = u8;
pub type AttackerDice = u8;
pub type DefenderDice = u8;

pub trait Holding {
    fn new(owner: Player, dice: u8, mobile: bool) -> Self;
    fn owner(&self) -> Player;
    fn dice(&self) -> u8;
    fn mobile(&self) -> bool;

    fn as_string(&self) -> String {
        if self.mobile() {
            format!("{}|{}", self.owner(), self.dice())
        } else {
            format!("{}#{}", self.owner(), self.dice())
        }
    }
}

impl Holding for u8 {
    fn new(owner: Player, dice: u8, mobile: bool) -> Self {
        let player_num = *owner.number() as u8;
        let player_num = player_num.to_le() << 5;
        let player_num = player_num >> 5;

        let dice = dice.to_le() << 5;
        let dice = dice >> 2;

        let mobile: u8 = mobile.into();
        let mobile = mobile.to_le() << 7;
        let mobile = mobile >> 1;

        u8::from_le(mobile | dice | player_num)
    }

    fn owner(&self) -> Player {
        let val = self.to_le() << 5;
        let val = val >> 5;
        player::create(u8::from_le(val) as usize)
    }

    fn dice(&self) -> u8 {
        let val = self.to_le() >> 3;
        let val = val << 5;
        let val = val >> 5;
        u8::from_le(val)
    }

    fn mobile(&self) -> bool {
        let val = self.to_le() << 1;
        let val = val >> 7;
        let boolean = u8::from_le(val);
        (true as u8) == boolean
    }
}

/// A territorial hold on a particular tile.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Hold {
    /// AKA the player.
    owner: Player,

    /// We're assuming D6's here.
    dice: u8,

    /// Barred from moving if false
    mobile: bool,
}

impl Holding for Hold {
    fn new(owner: Player, dice: u8, mobile: bool) -> Self {
        Hold { owner, dice, mobile }
    }

    fn owner(&self) -> Player {
        self.owner
    }

    fn dice(&self) -> u8 {
        self.dice
    }

    fn mobile(&self) -> bool {
        self.mobile
    }
}

impl fmt::Display for Hold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Default for Hold {
    fn default() -> Self {
        Hold::new(Player::default(), 0, true)
    }
}

/// The full state of the game. Represents an iteration of play.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Getters)]
pub struct Board {
    players: Players,
    //grid: Grid<Hold>,
    grid: Grid<u8>,
    captured_dice: u8,
    moved: u8,
}

impl Board {
    pub fn new(players: Players, grid: Grid<u8>, captured_dice: u8, moved: u8) -> Self {
        Board { players, grid, captured_dice, moved }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Little hack. Since we've switched to using a bit packed u8 instead of `Hold`,
        // it screws up the display. So we'll generate a once-off `Grid<Hold>` just
        // to print it to screen.
        let (columns, rows) = match self.grid.shape() {
            crate::hexagon::grid::Shape::Rectangular { columns, rows } => (columns, rows),
            _ => panic!("Display impl is a hack."),
        };
        
        let display_grid: Grid<Hold> = self.grid
            .iter()
            .map(|ht| (
                *ht.coordinate(),
                Hold::new(ht.data().owner(), ht.data().dice(), ht.data().mobile())
            ))
            .collect();
        let display_grid = display_grid.change_to_rectangle(columns, rows);
        
        write!(
            f,
            "Current Player: {}\nCaptured Dice: {}, Moved: {} time(s). \
             \nBoard =============\n{}",
            &self.players.current(),
            &self.captured_dice,
            &self.moved,
            &display_grid,
        )
    }
}

/// A legal player action that will advance the game state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Attack(FromHex, ToHex, AttackerDice, DefenderDice),
    Pass,
}

impl Action {
    /// Returns the amount of dice that will be captured by the move.
    pub fn capturing(&self) -> u8 {
        match self {
            Action::Attack(_, _, _, dd) => *dd,
            _ => 0,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Attack(from, to, _, capturing) => {
                write!(f, "Attack from {} into {} capturing {} dice.", from, to, capturing)
            },
            Action::Pass => write!(f, "Pass turn."),
        }
    }
}

/// What follows from a `Move`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consequence {
    Stalemate(Board),
    Continue(Board),
    TurnOver(Board),
    GameOver(Board),
    Winner(Board),
}

impl Consequence {
    pub fn board(&self) -> &Board {
        match self {
            Consequence::Stalemate(ref b) => b,
            Consequence::Continue(ref b) => b,
            Consequence::TurnOver(ref b) => b,
            Consequence::GameOver(ref b) => b,
            Consequence::Winner(ref b) => b
        }
    }
}

/// Scoring for each move. Present when AI calculations have been made.
#[derive(Debug, Copy, Clone, PartialEq, Getters)]
pub struct Score {
    /// The endstate board idealness. 1 means a win, 0 means loss and anything
    /// in between means a stalemate of different degrees.
    destination: f64,

    /// How far away yet still that endstate board is. The closer the better.
    distance: usize,
}

impl Score {
    pub fn new(destination: f64, distance: usize) -> Self {
        Score { destination, distance }
    }

    pub fn increment_distance(&self) -> Self {
        Score::new(self.destination, self.distance + 1)
    }
}

/// Custom impl since if the destination scores are equal, a smaller distance is better.
impl cmp::PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<cmp::Ordering> {
        if let Some(ordering) = self.destination.partial_cmp(&other.destination) {
            let ordering = match ordering {
                cmp::Ordering::Equal => {
                    // Closer is better. Thus here we invert.
                    if self.distance > other.distance {
                        cmp::Ordering::Less
                    } else if self.distance < other.distance {
                        cmp::Ordering::Greater
                    } else {
                        cmp::Ordering::Equal
                    }
                },
                _ => ordering,
            };
            Some(ordering)
        } else {
            None
        }
    }
}

/// Initialize with the worst possible score.
impl Default for Score {
    fn default() -> Self {
        Score::new(0_f64, 0)
    }
}

/// A `Choice` which that is an `Action` with its `Consequence`.
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    action: Action,
    consequence: Consequence,

    /// Filled in AI phase when scoring each move. 
    score: Cell<Option<Score>>,    
}

impl Choice {
    pub fn new(action: Action, consequence: Consequence) -> Self {
        Choice { action, consequence, score: Cell::new(None) }
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn consequence(&self) -> &Consequence {
        &self.consequence
    }

    pub fn score(&self) -> Option<Score> {
        self.score.get()
    }

    pub fn set_score(&self, score: Score) {
        self.score.set(Some(score));
    }

    pub fn clear_score(&self) {
        self.score.set(None);
    }
}

/// The game tree. Contains all moves possible from the starting state.
#[derive(Debug, Clone, Getters)]
pub struct Tree {
    root: Board,
    states: HashMap<Board, Vec<Choice>>,
}

impl Tree {
    pub (in crate::game) fn new(root: Board, states: HashMap<Board, Vec<Choice>>) -> Self {
        Tree { root, states }
    }

    pub (in crate::game) fn empty(root: Board) -> Self {
        Tree {
            root,
            states: HashMap::new(),
        }
    }

    /*
    pub (in crate::game) fn consume(self) -> (Board, HashMap<Board, Vec<Choice>>) {
        (self.root, self.states)
    }
     */

    pub (in crate::game) fn append(&mut self, extra: HashMap<Board, Vec<Choice>>) {
        extra
            .into_iter()
            .for_each(|(board, choices)| if !self.states.contains_key(&board) {
                self.states.insert(board, choices);
            });
    }
    
    /// Convenience method to save on calling the getters.
    pub fn fetch_choices(&self, board: &Board) -> Option<&[Choice]> {
        self.states.get(board).map(|v| v.as_slice())
    }
}

/// Some helpful information to gather during board generation to get an insight into
/// memory usage and geometric tree growth.
#[derive(Debug, Copy, Clone, Getters)]
pub struct LayerStats {
    /// How may layers deep this layer was at.
    depth: usize,

    /// The number of `Board`s (or states) that could be possible at this layer.
    boards: usize,

    /// The number of `Board`s that were unique and were inserted.
    inserted: usize,
}

impl LayerStats {
    pub fn new(depth: usize, boards: usize, inserted: usize) -> Self {
        LayerStats { depth, boards, inserted }
    }
}

impl fmt::Display for LayerStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ Depth: {}\t Boards: {}\t Inserted: {}\t Discarded: {}\t]",
            &self.depth,
            &self.boards,
            &self.inserted,
            self.boards - self.inserted,
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Totals {
    boards: usize,
    inserted: usize,
}

impl Totals {
    pub fn new(boards: usize, inserted: usize) -> Self {
        Totals { boards, inserted }
    }
}

impl ops::Add for Totals {
    type Output = Totals;
    
    fn add(self, rhs: Totals) -> Self::Output {
        Totals {
            boards: self.boards + rhs.boards,
            inserted: self.inserted + rhs.inserted,
        }
    }
}

impl<'a> ops::Add<&'a Totals> for Totals {
    type Output = Totals;
    
    fn add(self, rhs: &Totals) -> Self::Output {
        Totals {
            boards: self.boards + rhs.boards,
            inserted: self.inserted + rhs.inserted,
        }
    }
}

impl Default for Totals {
    fn default() -> Self {
        Totals::new(0, 0)
    }
}

impl fmt::Display for Totals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let discarded = self.boards - self.inserted;
        write!(
            f,
            " TOTALS = [ Boards Calculated: {}\t Inserted: {}\t Discarded: {}\t Efficiency:\
             {:.2}% ]",
            &self.boards,
            &self.inserted,
            &discarded,
            if self.boards == 0 || self.inserted == 0 {
                0_f64
            } else {
                self.inserted as f64 / self.boards as f64 * 100_f64
            },
        )
    }
}

#[cfg(test)]
mod test {
    use std::error;

    use crate::game;
    use super::super::{build_tree, Player};
    use super::*;

    #[test]
    fn board_matches_board_2x1() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x1_start01();
        let tree = build_tree(start.clone(), 1);

        assert!(tree.root == start);

        Ok(())
    }
    
    #[test]
    fn board_matches_board_2x2() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x2_start01();
        let tree = build_tree(start.clone(), 1);

        assert!(tree.root == start);

        Ok(())
    }

    #[test]
    fn u8_holding_01() -> Result<(), Box<dyn error::Error>> {
        let player1 = Player::new(1, 'A');
        let holding = u8::new(player1, 2, true);

        assert!(holding.owner() == player1);
        assert!(holding.dice() == 2);
        assert!(holding.mobile() == true);

        Ok(())
    }

    #[test]
    fn u8_holding_02() -> Result<(), Box<dyn error::Error>> {
        let player1 = Player::new(5, 'E');
        let holding = u8::new(player1, 1, false);

        assert!(holding.owner() == player1);
        assert!(holding.dice() == 1);
        assert!(holding.mobile() == false);

        Ok(())
    }

    #[test]
    fn u8_holding_03() -> Result<(), Box<dyn error::Error>> {
        let player1 = Player::new(6, 'F');
        let holding = u8::new(player1, 5, true);

        dbg!(holding.owner());
        assert!(holding.owner() == player1);
        assert!(holding.dice() == 5);
        assert!(holding.mobile() == true);

        Ok(())
    }
}
