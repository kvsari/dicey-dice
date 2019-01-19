//! The tree containing all the legal moves in a game.
use std::collections::HashMap;
use std::iter::Extend;
use std::fmt;

use derive_getters::Getters;

use crate::hexagon::{coordinate, grid};
use super::{
    rules,
    Grid,
    hold::Hold,
    player::{Player, Players},
};

type FromHex = coordinate::Cube;
type ToHex = coordinate::Cube;
type StateIndex = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Getters)]
pub struct BoardState {
    players: Players,
    grid: Grid,
}

impl BoardState {
    pub fn new(players: Players, grid: Grid) -> Self {
        BoardState { players, grid }
    }

    /// Create a copy of self with only the `grid` updated. Current player remains same.
    pub fn update_grid(&self, grid: Grid) -> Self {
        BoardState {
            players: self.players,
            grid,
        }
    }

    /*
    /// Pass in the winning function
    fn winning_with<F: Fn(&Grid, &Player) -> bool>(&self, f: F) -> bool {
    }
    */
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Current Player: {}\nBoard ====\n{}",
            &self.players.current(),
            &self.grid,
        )
    }
}

/// A legal move from one `BoardState` into another.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Attack(FromHex, ToHex),
    Pass,
}

/// What follows from a `Move`.
#[derive(Debug, Clone)]
pub enum Consequence {
    Continue(BoardState),
    TurnOver(BoardState),
    GameOver(BoardState),
    Winner,
}

impl Consequence {
    fn boardstate(&self) -> Option<&BoardState> {
        match self {
            Consequence::Continue(ref b) => Some(b),
            Consequence::TurnOver(ref b) => Some(b),
            Consequence::GameOver(ref b) => Some(b),
            Consequence::Winner => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Next {
    movement: Move,
    consequence: Consequence,
}

impl Next {
    pub fn new(movement: Move, consequence: Consequence) -> Self {
        Next { movement, consequence }
    }
}

/*
/// A traversal through the tree.
pub struct Traversal {
}
 */

/// The game tree. Contains all moves possible from the starting state.
#[derive(Debug, Clone)]
pub struct Tree {
    players: Players,
    start: BoardState,
    traversal: Vec<BoardState>,
    states: HashMap<BoardState, Vec<Next>>,
}

impl Tree {
    pub fn current_traversal(&self) -> &BoardState {
        self.traversal.last().unwrap()
    }
}

/// Generate a full grame decision free encompassing all possible legal moves starting
/// from the current player returned by `players`.
pub fn grow_entire_tree_from(grid: Grid, players: Players) -> Tree {
    let starting_state = BoardState::new(players, grid);

    Tree {
        players,
        start: starting_state.clone(),
        traversal: vec![starting_state.clone()],
        states: calculate_all_consequences(starting_state),
    }
}

/// Function will build all boardstates from `start`  inserting them into the `states` map.
/// If the boardstate already exists will skip that boardstate. This function has no
/// horizon so it won't stop generating until the stack is empty.
fn calculate_all_consequences(start: BoardState) -> HashMap<BoardState, Vec<Next>> {
    let mut stack: Vec<BoardState> = Vec::new();
    let mut states: HashMap<BoardState, Vec<Next>> = HashMap::new();
    
    // 1. Seed tree generation by pushing the first boardstate onto the stack.
    stack.push(start);

    // 2. Get the next boardstate off the stack.
    while let Some(board) = stack.pop() {
        // 3. Check if the board hasn't been stored yet. If it has we skip.
        if !states.contains_key(&board) {           
            // 4. If not, we calculate the `Next`s.
            let nexts = rules::boardstate_consequences(&board);

            // 5. We then push all the resulting boardstates onto the stack.
            stack.extend(
                nexts
                    .iter()
                    .filter_map(|next| next.consequence.boardstate())
                    .map(|board| board.clone())
            );

            // 6. Then we insert the boardstate and `Next`s into the map.
            states.insert(board, nexts);
        }
    }

    // 7. Return results of traversal.
    states
}
