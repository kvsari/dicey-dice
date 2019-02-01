//! Handle a game.
use derive_getters::Getters;

use crate::game::{self, Tree, Board, Players, Player, Choice};

/// Setup a game session. Can set the number of players and the board size and to use
/// canned boards (feed it a starting position. The board can only be rectangular.
#[derive(Debug, Clone, Getters)]
pub struct Setup {
    players: Players,
    board: Option<Board>,
}

impl Setup {
    pub fn new(players: Players) -> Self {
        Setup {
            players,
            board: None,
        }
    }

    /// If the number of players changes in any way, it will invalidate the `Board`.
    pub fn set_players(&mut self, players: Players) -> &mut Self {        
        if self.players != players {
            self.board = None;
        }
        self.players = players;
        self
    }    

    /// Set the board. This will also set the players since the `Board` lists all state.
    pub fn set_board(&mut self, board: Board) -> &mut Self {
        self.players = *board.players();
        self.board = Some(board);
        self
    }

    /// Will generate a new board using the loaded in `Players` setting.
    pub fn gen_board(&mut self, columns: u32, rows: u32) -> &mut Self {
        self.board = Some(game::generate_random_board(columns, rows, self.players));
        self
    }

    /// Produce a game session! Will return an error if there is no `Board` setup. Boards
    /// greater than 3x3 will hang the system as the current state of the library is to
    /// 'solve' the game by resolving the entire tree of every possible action.
    pub fn session(&self) -> Result<Session, String> {
        if let Some(board) = self.board.clone() {
            let tree: Tree = board.clone().into();
            Ok(Session::new(board, tree))
        } else {
            Err("No board set.".to_owned())
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        Setup::new(Players::new(2))
    }
}

/// A game in progress. The `traversals` indicate how many turns have passed. Maintains
/// all state of the game.
///
/// ## Invariants
/// 1. The `tree` will always be valid.
/// 2. The first `Board` in the `traversal` is the starting position.
/// 3. There will always be at least one `Board` in the `traversal`.
#[derive(Debug, Clone, Getters)]
pub struct Session {
    traversal: Vec<Board>,
    tree: Tree,
    choice: Option<Choice>,
}

impl Session {
    pub fn new(start: Board, tree: Tree) -> Self {
        Session {
            traversal: vec![start],
            tree,
            choice: None,
        }
    }

    /// Return the current `Board`.
    pub fn board(&self) -> &Board {
        self.traversal.last().unwrap()
    }

    /// Return the current `Player`. Can also be sourced from the `Board`.
    pub fn player(&self) -> Player {
        self.traversal.last().unwrap().players().current()
    }

    /// Choices available from the current `Board`.
    pub fn choices(&self) -> &[Choice] {
        self.tree.fetch_choices(self.board()).unwrap()
    }

    /// Lock in a `Choice`.
    pub fn choose(&mut self, choice: Choice) -> Result<(), String> {
        Err("Invalid choice.".to_owned())
    }

    /*
    /// Advance the state of the `Session`. If a choice has been made, will apply it. If
    /// no choice has been made, perhaps the game can be advanced anyway such as a turn over
    /// or a player `GameOver`. Returns `true` if the game advanced.
    pub fn advance(&mut self) -> bool {
        if 
    }
    */
}
