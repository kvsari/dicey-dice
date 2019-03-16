//! Handle a game.
use std::num::NonZeroU8;
use std::fmt;

use derive_getters::Getters;
use rand::{rngs, Rng};

use crate::game::{self, Tree, Board, Players, Player, Choice, Action, Consequence, Holding};

fn roll_d6s<T: Rng>(d6s: u8, random: &mut T) -> usize {
    (0..d6s)
        .fold(0, |sum, _| -> usize {
            sum + random.gen_range(1, 7)
        })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LastAttack {
    attacker_dice: u8,
    attacker_rolled: usize,
    defender_dice: u8,
    defender_rolled: usize,
}

impl LastAttack {
    fn new(
        attacker_dice: u8, attacker_rolled: usize, defender_dice: u8, defender_rolled: usize
    ) -> Self {
        LastAttack { attacker_dice, attacker_rolled, defender_dice, defender_rolled }
    }
}

impl fmt::Display for LastAttack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attacker_rolled == 0 && self.defender_rolled == 0 {
            write!(f, "") // Sentinel value for first turn thus no preceding attacks.
        } else {
            if self.attacker_rolled > self.defender_rolled {
                write!(
                    f,
                    "Attacker with {} dice rolled {} beating \
                     defender with {} dice who rolled {}.",
                    &self.attacker_dice,
                    &self.attacker_rolled,
                    &self.defender_dice,
                    &self.defender_rolled,
                )
            } else {
                write!(
                    f,
                    "Defender with {} dice rolled {} holding against \
                     attacker with {} dice who rolled {}.",
                    &self.defender_dice,
                    &self.defender_rolled,
                    &self.attacker_dice,
                    &self.attacker_rolled,
                )
            }
        }
    }
}

impl Default for LastAttack {
    fn default() -> Self {
        LastAttack::new(0, 0, 0, 0)
    }
}

/// State of game progression. Whether the game is on, over and what kind of over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progression {
    PlayOn(LastAttack),
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
fn state_from_board(
    mut current_board: Board, tree: &Tree, outcome: LastAttack,
) -> Result<State, usize> {
    let mut traversal: Vec<(Board, Choice)> = Vec::new();   
    let mut depth: usize = 1;
    
    let state = loop {
        let choices = tree
            .fetch_choices(&current_board)
            .ok_or(depth)?;
        
        // If there's only one choice left, it may be a passing/gameover/win move. Or the
        // last available attack.
        if choices.len() == 1 {
            depth += 1;
            match choices[0].action() {
                Action::Attack(_, _, _, _) => {
                    // There is one last attack to make. We won't execute this choice
                    // for the player as that'd be overstepping our bounds. Thus we jump
                    // out of this loop.
                    break State::new(
                        Progression::PlayOn(outcome),
                        traversal.as_slice(),
                        current_board,
                        choices,
                    );
                },
                Action::Pass => {
                    // It'd be cumbersome to manually pass a move. The player can't "do"
                    // anything. So let's just deal with it automatically.

                    // In order to do this, we need to figure out the passing consequence.
                    match choices[0].consequence() {
                        Consequence::Stalemate(next_board) => break State::new(
                            Progression::GameOverStalemate(next_board.players().playing()),
                            traversal.as_slice(),
                            next_board.to_owned(),
                            choices,
                        ),
                        Consequence::Winner(next_board) => break State::new(
                            Progression::GameOverWinner(next_board.players().current()),
                            traversal.as_slice(),
                            next_board.to_owned(),
                            choices,
                        ),
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
                        Consequence::Continue(_) => unreachable!(),
                    }
                },
            }
        }

        // If we make it here, there are choices that need to be made.
        break State::new(
            Progression::PlayOn(outcome),
            traversal.as_slice(),
            current_board,
            choices,
        );
    };

    Ok(state)
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
    tree: Option<Tree>,
    move_limit: NonZeroU8,
    rand: rngs::ThreadRng,
}

impl Session {
    pub fn new(start: Board, tree: Tree, move_limit: NonZeroU8) -> Self {
        // The start may contain pass move. Cycle to get at the first true turn.
        // This code is a copy of what's happening in `advance` below. TODO: Refactor me.
        
        let mut tree = Some(tree);
        let first_turn = loop {
            match state_from_board(
                start.clone(), tree.as_ref().unwrap(), LastAttack::default()
            ) {
                Ok(state) => break state,
                Err(depth) => {
                    let new_tree = game::start_tree_horizon_limited(
                        start.clone(), depth, move_limit.get(),
                    );
                    tree = Some(new_tree);
                },
            }
        };
        
        Session {
            turns: vec![first_turn],
            tree,
            move_limit,
            rand: rand::thread_rng(),
        }
    }

    pub fn reset(self) -> Self {
        let first = self.turns.first().unwrap().board.to_owned();
        Session::new(
            first.clone(),
            game::start_tree_horizon_limited(first, 1, self.move_limit.get()),
            self.move_limit,
        )
    }
            
    pub fn current_turn(&self) -> &State {
        self.turns.last().unwrap()
    }

    /// Take an `Action` and advance the game state. Advances the tree if necessary. Takes
    /// an `index` of the `[Choice]`. The `Choice` will always be an attacking action.
    pub fn advance(&mut self, index: usize) -> Result<&State, String> {
        let choice = self
            .current_turn()
            .choices()
            .get(index)
            .ok_or("Index out of bounds.".to_owned())?
            .to_owned();

        let (attacker_coordinate, attacker_dice, defender_dice) = match choice.action() {
            Action::Attack(ac, _, ad, dd) => (*ac, *ad, *dd),
            Action::Pass => unreachable!(), // Must never happen. `Session` must always
                                            // return with attack choices or game over.
        };

        let attacker_roll = roll_d6s(attacker_dice, &mut self.rand);
        let defender_roll = roll_d6s(defender_dice, &mut self.rand);

        let outcome = LastAttack::new(
            attacker_dice, attacker_roll, defender_dice, defender_roll
        );
        
        let next_board = if attacker_roll > defender_roll {
            // Board advances due to win.
            choice.consequence().board().to_owned()
        } else {
            // Board stays the same sans one move due to loss and the losing hex frozen.
            let current_board = &self.current_turn().board;
            Board::new(
                *current_board.players(),
                current_board
                    .grid()
                    .fork_with(|coordinate, hold| {
                        // Freeze the losing hex til next turn.
                        if coordinate == &attacker_coordinate {
                            game::model::Hold::new(hold.owner(), hold.dice(), false)
                        } else {
                            hold
                        }
                    }),
                *current_board.captured_dice(),
                *current_board.moved() + 1,
            )
        };
        
        let state = loop {
            match state_from_board(
                next_board.clone(), &self.tree.as_ref().unwrap(), outcome,
            ) {
                Ok(state) => break state,
                Err(depth) => {
                    let new_tree = game::start_tree_horizon_limited(
                        next_board.to_owned(), depth, self.move_limit.get(),
                    );
                    self.tree = Some(new_tree);
                },
            }
        };
        
        self.turns.push(state);
        Ok(self.current_turn())
    }

    /// Score the tree up to the depth specified in `horizon`. Will then edit current
    /// `State` to put the scoring into the current choices. A deep horizon will cause the
    /// system to lock up. High chance that an OOM error will follow.
    pub fn score_with_depth_horizon(&mut self, horizon: usize) -> &State {
        let current_board = self.current_turn().board.to_owned();
        let tree = game::start_tree_horizon_limited(
            current_board, horizon, self.move_limit.get(),
        );
        
        let _ = game::score_tree(&tree);
        let choices = tree.fetch_choices(tree.root()).unwrap().to_owned();
        let last_state = self.turns.last_mut().unwrap();
        last_state.choices = choices;
        self.tree = Some(tree);
        last_state
    }

    /// Score the tree up to the the board insert budget specified. The first tree layer
    /// though will be computed without taking into account the budget, this way there will
    /// always be all available choices for the turn.
    pub fn score_with_insert_budget(&mut self, insert_budget: usize) -> &State {
        let current_board = self.current_turn().board.to_owned();
        let tree = game::start_tree_insert_budgeted(
            current_board, insert_budget, self.move_limit.get(),
        );
        
        let _ = game::score_tree(&tree);
        let choices = tree.fetch_choices(tree.root()).unwrap().to_owned();
        let last_state = self.turns.last_mut().unwrap();
        last_state.choices = choices;
        self.tree = Some(tree);
        last_state
    }
}

/// Setup a game session. Can set the number of players and the board size and to use
/// canned boards (feed it a starting position. The board can only be rectangular.
#[derive(Debug, Clone, Getters)]
pub struct Setup {
    players: Players,
    board: Option<Board>,
    move_limit: NonZeroU8,
}

impl Setup {
    pub fn new() -> Self {
        Setup {
            players: Players::new(2),
            board: None,
            move_limit: NonZeroU8::new(6).unwrap(),
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

    pub fn set_move_limit(&mut self, move_limit: NonZeroU8) -> &mut Self {
        self.move_limit = move_limit;
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
            let tree = game::start_tree_horizon_limited(
                board.clone(), 1, self.move_limit.get());
            Ok(Session::new(board, tree, self.move_limit))
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

#[cfg(test)]
mod test {
    use std::error;

    use crate::{game, session};
    
    use super::*;

    #[test]
    fn state_from_board_2x1() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x1_start01();
        let s_grid = start.grid().to_owned();
        let tree = game::build_tree(start.clone());

        let state = state_from_board(&start, &tree).unwrap();
        let f_grid = state.board().grid().to_owned();

        assert!(s_grid == f_grid);

        Ok(())
    }

    #[test]
    fn state_from_board_2x2() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x2_start01();
        let s_grid = start.grid().to_owned();
        let tree = game::build_tree(start.clone());

        let state = state_from_board(&start, &tree).unwrap();
        let f_grid = state.board().grid().to_owned();

        assert!(s_grid == f_grid);

        Ok(())
    }

    #[test]
    fn start_grid_matches_2x1() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x1_start01();
        let s_grid = start.grid().to_owned();

        let session = session::Setup::new()
            .set_board(start)
            .session()?;

        let state = session.current_turn().to_owned();
        let f_grid = state.board().grid().to_owned();

        assert!(s_grid == f_grid);

        Ok(())
    }

    #[test]
    fn start_grid_matches_2x2() -> Result<(), Box<dyn error::Error>> {
        let start = game::canned_2x2_start01();
        let s_grid = start.grid().to_owned();

        let session = session::Setup::new()
            .set_board(start)
            .session()?;

        let state = session.current_turn().to_owned();
        let f_grid = state.board().grid().to_owned();

        assert!(s_grid == f_grid);

        Ok(())
    }
}
