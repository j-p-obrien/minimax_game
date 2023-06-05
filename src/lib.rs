pub mod tic_tac_toe;

use std::fmt::Display;

use rand::seq::SliceRandom;

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

/// The state of the game. This should include both the current board position and any other
/// necessary information e.g. in chess, we want this to include some kind of history so we
/// can determine things like the 3-move repetition rule.
pub trait GameState: Clone {
    /// This type should describe the moves of the game.
    type Move: Clone;

    /// Returns a new game, starting from the beginning board state.
    fn new() -> Self;
    /// Returns a Vec of all the legal moves based on the current game state.
    fn get_legal_moves(&self) -> Vec<Self::Move>;
    /// Tries to apply the given move to advance the GameState. Returns true if the move was legal;
    /// false otherwise.
    fn try_move(&mut self, move_candidate: &Self::Move) -> bool;
    /// Returns the current result of the game
    fn game_result(&self) -> GameResult;
}

/// This trait is used to evaluate the strength of a player's position on the board. It can do
/// things like compute a Q value for Q-learning, a policy, or really any kind of useful
/// information that can be used to make decisions in the game e.g. in AlphaZero, this would
/// compute both Q for the reachable positions and a preliminary policy for these positions too.
/// The actual MCTS should be handled by a struct that implements Strategy<G>.
pub trait Evaluator<G: GameState> {
    type Evaluation;

    fn new() -> Self;
    fn evaluate(&self, state: &G) -> Self::Evaluation;
}

/// Useful struct for when you do not need to evaluate how advantageous a position is e.g. you are
/// playing randomly.
pub struct EmptyEvaluator;
impl<G> Evaluator<G> for EmptyEvaluator
where
    G: GameState,
{
    type Evaluation = ();

    fn new() -> Self {
        EmptyEvaluator
    }

    fn evaluate(&self, _state: &G) -> Self::Evaluation {
        ()
    }
}

/// This trait is used to actually compute the move taken given the current state of the game. It
/// is intended that structs implementing this trait use information provided by the evaluator to
/// make its decision e.g. in AlphaZero, this trait would involve the computation of the MCTS
/// before making the actual move.
pub trait Strategy<G: GameState> {
    type Evaluator: Evaluator<G>;

    fn new() -> Self;
    fn choose_move(&self, state: &G) -> Option<G::Move>;
}

pub struct RandomStrat;

impl<G> Strategy<G> for RandomStrat
where
    G: GameState,
{
    type Evaluator = EmptyEvaluator;

    fn new() -> Self {
        RandomStrat
    }

    fn choose_move(&self, state: &G) -> Option<G::Move> {
        let legal_moves = state.get_legal_moves();
        let random_move = legal_moves.choose(&mut rand::thread_rng());
        random_move.cloned()
    }
}

#[derive(Debug)]
pub struct GamePlayer<G, E, S>
where
    G: GameState,
    E: Evaluator<G>,
    S: Strategy<G>,
{
    state: G,
    evaluator: E,
    strategy: S,
}

impl<G, E, S> GamePlayer<G, E, S>
where
    G: GameState + Display,
    E: Evaluator<G>,
    S: Strategy<G>,
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
                        self.state.try_move(&move_candidate);
                        std::thread::sleep(std::time::Duration::from_secs(1))
                    }
                }
                result @ GameResult::Win(player) => {
                    println!("{:?} wins!", player);
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
