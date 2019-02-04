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
#[derive(Debug, Clone, Getters)]
pub struct State {
    /// Whether we continue play or not.
    game: Progression,
    
    /// If a bunch of single move turns needed to be made first. These include players
    /// being knocked out or players only being able to pass their turn.
    traversal: Vec<(Board, Choice)>,

    /// State of game.
    board: Board,

    /// Choices available to current player.
    choices: Vec<Choice>,
}

impl State {
    fn new(
        game: Progression,
        traversal: &[(Board, Choice)],
        board: Board,
        choices: &[Choice],
    ) -> Self {
        State {
            game,
            traversal: traversal
                .into_iter()
                .map(|(b, c)| (b.to_owned(), c.to_owned()))
                .collect(),
            board,
            choices: choices
                .into_iter()
                .map(|c| c.to_owned())
                .collect(),
        }
    }
}

/// Generate a `State` from a chosen `Board` consequence and the `Tree` where that `Board`
/// must exist. Runs inside a loop skipping over states that have only one turn left in
/// them except for Winning states. Uses some logic to detect draw states.
fn state_from_board(board: &Board, tree: &Tree) -> State {
    let mut traversal: Vec<(Board, Choice)> = Vec::new();   
    let mut current_board = board.to_owned();
    
    let state = loop {
        let choices = tree.fetch_choices(&current_board).unwrap();
        // If there's only one choice left, it may be a passing/gameover/win move. Or the
        // last available attack.
        if choices.len() == 1 {
            match choices[0].consequence() {
                Consequence::Winner(next_board) => {
                    // TODO: Generate a `State` with the right game progression.
                    break State::new(
                        Progression::GameOverWinner(next_board.players().current()),
                        traversal.as_slice(),
                        next_board.to_owned(),
                        choices,
                    );
                },
                Consequence::GameOver(next_board) => {
                    // We need to iterate the progression.
                    traversal.push((current_board, choices[0].to_owned()));
                    current_board = next_board.to_owned();
                    continue;
                },
                Consequence::TurnOver(next_board) => {
                    // We need to iterate the progression.
                    traversal.push((current_board, choices[0].to_owned()));
                    current_board = next_board.to_owned();
                    continue;
                },
                Consequence::Continue(next_board) => {
                    // Generate a `State` with the single choice for progression.
                    break State::new(
                        Progression::PlayOn,
                        traversal.as_slice(),
                        next_board.to_owned(),
                        choices,
                    );
                },
            }
        }

        // If we make it here, there is a legit choice that needs to be made.
        break State::new(
            Progression::PlayOn,
            traversal.as_slice(),
            current_board,
            choices,
        );
    };

    state
}
    
/// A game in progress. The `traversals` indicate how many turns have passed. Maintains
/// all state of the game.
///
/// ## Invariants
/// 1. The `Tree` will always be valid.
/// 2. The first `State` in the `turns` is the starting position sans any inital traversals.
/// 3. There will always be at least one `State` in the `turns`.
#[derive(Debug, Clone, Getters)]
pub struct Session {
    turns: Vec<State>,
    tree: Tree,
}

impl Session {
    pub fn new(start: Board, tree: Tree) -> Self {
        Session {
            turns: vec![state_from_board(&start, &tree)],
            tree,
        }
    }

    pub fn reset(self) -> Self {
        let first = self.turns.first().unwrap().to_owned();
        Session {
            turns: vec![first],
            tree: self.tree,
        }
    }
            
    pub fn current_turn(&self) -> &State {
        self.turns.last().unwrap()
    }

    /// Take an `Action` and advance the game state.
    pub fn advance(&mut self, choice: &Choice) -> Result<&State, String> {
        let state = self.current_turn();
        for available_choice in state.choices.iter() {
            if available_choice == choice {
                let board = choice.consequence().board();
                let state = state_from_board(board, &self.tree);
                self.turns.push(state);
                return Ok(self.current_turn());
            }
        }

        Err("Invalid action.".to_owned())
    }
}

/// Setup a game session. Can set the number of players and the board size and to use
/// canned boards (feed it a starting position. The board can only be rectangular.
#[derive(Debug, Clone, Getters)]
pub struct Setup {
    players: Players,
    board: Option<Board>,
}

impl Setup {
    pub fn new() -> Self {
        Setup {
            players: Players::new(2),
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
        Setup::new()
    }
}
