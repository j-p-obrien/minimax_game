use crate::{evaluate::*, strategy::*};
use std::fmt::Display;

/// Used to represent which player is going.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Player {
    #[default]
    One,
    Two,
}

/// Represents the current outcome of the game. Undetermined denotes a non-terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameResult {
    Win(Player),
    Draw,
    #[default]
    Undetermined,
}

#[derive(Debug)]
pub struct GamePlayer<G, E, S>
where
    G: GameState,
    E: Evaluator<G>,
    S: Strategy<G, E>,
{
    state: G,
    evaluator: E,
    strategy: S,
}

/// The state of the game. This should include both the current board position and any other
/// necessary information e.g. in chess, we want this to include some kind of history so we
/// can determine things like the 3-move repetition rule.
pub trait GameState: Sized {
    /// This type should describe the moves of the game.
    type Move;

    /// Returns a new game, starting from the beginning board state.
    fn new() -> Self;

    /// Returns a Vec of all the legal moves based on the current game state.
    fn get_legal_moves(&self) -> Vec<Self::Move>;

    /// Tries to apply the given move to advance the GameState. Returns true if the move was legal;
    /// false otherwise.
    fn apply_move(&mut self, mov: &Self::Move);

    /// Returns what the next game state would be if the move were applied.
    fn next_state(&self, mov: &Self::Move) -> Self;

    /// Returns the current result of the game
    fn game_result(&self) -> GameResult;

    /// Returns a vector of game states reachable from the current state in one move.
    fn reachable_states(&self) -> Vec<Self> {
        self.get_legal_moves()
            .into_iter()
            .map(|mov| self.next_state(&mov))
            .collect()
    }

    /// Returns a vector of tuples (state, move), where state would be the next game state if move
    /// were applied.
    fn get_state_moves(&self) -> Vec<(Self, Self::Move)> {
        self.get_legal_moves()
            .into_iter()
            .map(|mov| (self.next_state(&mov), mov))
            .collect()
    }
}

impl Player {
    /// Returns the other Player enum variant
    pub fn other_player(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    /// Flips the player in-place.
    pub fn flip_player(&mut self) {
        match *self {
            Player::One => *self = Player::Two,
            Player::Two => *self = Player::One,
        };
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Player::One => write!(f, "Player One")?,
            Player::Two => write!(f, "Player Two")?,
        }
        Ok(())
    }
}

impl From<Player> for GameResult {
    /// Turns the given player into Win(player).
    fn from(value: Player) -> Self {
        match value {
            Player::One => Self::Win(Player::One),
            Player::Two => Self::Win(Player::Two),
        }
    }
}

impl<G, E, S> GamePlayer<G, E, S>
where
    G: GameState + Display,
    E: Evaluator<G>,
    S: Strategy<G, E>,
{
    pub fn new() -> GamePlayer<G, E, S> {
        GamePlayer {
            state: GameState::new(),
            evaluator: Evaluator::new(),
            strategy: Strategy::new(),
        }
    }

    pub fn from(state: G, evaluator: E, strategy: S) -> GamePlayer<G, E, S> {
        GamePlayer {
            state,
            evaluator,
            strategy,
        }
    }

    pub fn play(&mut self) -> GameResult {
        loop {
            print!("{}", &self.state);
            match self.state.game_result() {
                GameResult::Undetermined => {
                    if let Some(move_candidate) = self.strategy.choose_move(&self.state) {
                        self.state.apply_move(&move_candidate);
                        std::thread::sleep(std::time::Duration::from_secs(1))
                    }
                }
                result @ GameResult::Win(player) => {
                    println!("{} wins!", player);
                    return result;
                }
                result @ GameResult::Draw => {
                    println!("Game ended in a draw.");
                    return result;
                }
            }
        }
    }
}
