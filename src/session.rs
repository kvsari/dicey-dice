//! Handle a game.
use derive_getters::Getters;

use crate::game::{self, Tree, Board, Players, Player, Choice, Consequence};

/// State of game progression. Whether the game is on, over and what kind of over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progression {
    PlayOn,
    GameOverWinner(Player),
    GameOverStalemate(Vec<Player>), // Easier to calculate than a draw...
}

/// The state of the session.
#[derive(Debug, Clone)]
pub struct State {
    game: Progression,
    traversal: Vec<(Board, Choice)>,
    current_board: Board,
    current_choices: Vec<Choice>,
}

impl State {
    fn new(game: Progression, /*turn: usize,*/ traversal: &[(Board, Choice)]) -> Self {
        State {
            game,
            //turn,
            traversal: traversal.into_iter().map(|(b, c)| (b.to_owned(), *c)).collect(),
        }
    }
}

/*
/// Create the first `State` struct. Assumes the `Board` is the starting board that was
/// used to generate the entire `Tree`.
fn starting_state(board: &Board, tree: &Tree) -> Result<State, String> {
    if let Some(choices) = tree.get(board) {
        
    } else {
        Err("Board not present in tree.".to_owned())
    }
}
 */

/// Generate a `State` from a chosen `Board` consequence and the `Tree` where that `Board`
/// must exist. Runs inside a loop skipping over states that have only one turn left in
/// them except for Winning states. Uses some logic to detect draw states.
fn state_from_consequence(board: &Board, tree: &Tree) -> State {
    let mut traversal: Vec<(Board, Choice)> = Vec::new();
    
    //let choices = tree.get(board).unwrap();

    let mut current_board = board.to_owned();
    
    loop {
        let choices = tree.get(&current_board).unwrap();
        // If there's only one choice left, it may be a passing/gameover/win move. Or the
        // last available attack.
        if choices.len() == 1 {
            match choices[0].consequence() {
                Consequence::Winner(next_board) => {
                    // TODO: Generate a `State` with the right game progression.
                },
                Consequence::GameOver(next_board) => {
                    // TODO: We need to iterate the progression.
                },
                Consequence::TurnOver(next_board) => {
                    // TODO: We need to iterate the progression.
                },
                Consequence::Continue(next_board) => {
                    // TODO: Generate a `State` with the single choice for progression.
                },
            }
        }        

        // If we make it here, there is a legit choice that needs to be made.
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
    traversal: Vec<State>,
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

    pub fn reset(self) -> Self {
        Session {
            traversal: vec![self.traversal.first().take().unwrap().to_owned()],
            tree: self.tree,
            choice: None,
        }
    }
            
    //pub fn current(&self) -> 
    
    /*
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

    /// Advance the state of the `Session`. If a choice has been made, will apply it. If
    /// no choice has been made, perhaps the game can be advanced anyway such as a turn over
    /// or a player `GameOver`. Returns `true` if the game advanced.
    pub fn advance(&mut self) -> bool {
        if 
    }
    */
}

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
