use crate::game::*;

/// TODO: This may change in the future, but is fine for now.
type Probability = f32;

/// The return type of an evaluator that computes Q-values.
pub trait QValue {
    type Q: Sized;

    /// Returns the Q-value
    fn q(&self) -> Self::Q;
}

/// The return type of an evaluator that computes the probabilities of a win/draw/loss.
pub trait ResultDistribution: Sized {
    /// The probability of a win.
    fn win_prob(&self) -> Probability;
    /// The probabilty of a loss.
    fn draw_prob(&self) -> Probability;
    /// The probabilty of a draw.
    fn loss_prob(&self) -> Probability;
    /// Flips the probability of win/loss. Often, especially if you are using a tree search, it is
    /// useful to flip the perspective of the evaluation.
    fn other_perspective(&self) -> Self;
    /// Returns the expected value of the outcome of the game. Computed like win=1, draw=0, loss=1.
    fn expected_result(&self) -> Probability {
        self.win_prob() - self.loss_prob()
    }
}

/// The return type of an evaluator that computes a policy over the available moves.
pub trait Policy<G>
where
    G: GameState,
{
    /// Computes a probability distribution over all (legal) moves.
    fn policy(&self) -> Vec<(G::Move, Probability)>;
}

/// This trait is used to evaluate the strength of a player's position on the board. It can do
/// things like compute a Q value for Q-learning, a policy, or really any kind of useful
/// information that can be used to make decisions in the game e.g. in AlphaZero, this would
/// compute both Q for the reachable positions and a preliminary policy for these positions too.
/// The actual MCTS should be handled by a struct that implements Strategy<G, E>.
pub trait Evaluator<G>: Sized
where
    G: GameState,
{
    /// Intended to be one of the above types. Not necessary though!
    type Evaluation;

    /// Returns a new evaluator.
    fn new() -> Self;

    /// Evaluates the reward/favorability of the given mov for the given state, from the current
    /// player's perspective, given the strategy. If you are returning a policy, this is usually calculated from the
    /// only the game state.
    fn evaluate(&self, state: &G, mov: &G::Move) -> Self::Evaluation;
}

/// Typical example of a struct that implements ResultDistribution.
#[derive(Debug, Clone, Copy)]
pub struct Distribution {
    /// Probabilities of win/loss, respectively.
    probs: [Probability; 2],
}

impl Distribution {
    pub fn flip_perspective(&mut self) {
        self.probs = [self.probs[1], self.probs[0]]
    }

    pub fn win() -> Self {
        Self { probs: [1.0, 0.0] }
    }

    pub fn loss() -> Self {
        Self { probs: [0.0, 1.0] }
    }

    pub fn draw() -> Self {
        Self { probs: [0.0, 0.0] }
    }
}

impl ResultDistribution for Distribution {
    fn win_prob(&self) -> Probability {
        self.probs[0]
    }

    fn draw_prob(&self) -> Probability {
        1.0 - self.probs[0] - self.probs[1]
    }

    fn loss_prob(&self) -> Probability {
        self.probs[1]
    }

    fn other_perspective(&self) -> Self {
        Self {
            probs: [self.probs[1], self.probs[0]],
        }
    }
}

pub struct TerminalStateEvaluator;

impl<G> Evaluator<G> for TerminalStateEvaluator
where
    G: GameState,
{
    type Evaluation = GameResult;

    fn new() -> Self {
        Self
    }

    fn evaluate(&self, state: &G, mov: &<G as GameState>::Move) -> Self::Evaluation {
        let current_player = state.current_player();
        let other_player = current_player.other_player();
        let next_state = state.next_state(mov);
        let game_result = next_state.game_result();

        if game_result.is_determined() {
            return game_result;
        }
        //if let Some(opponents_move)

        todo!()
    }
}
