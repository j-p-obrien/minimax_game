use crate::{game::*, strategy::*};

/// The return type of an evaluator that computes Q-values.
pub trait QValue {
    type Q: Sized;

    fn q(&self) -> Self::Q;
}

/// The return type of an evaluator that computes the probabilities of a win/draw/loss.
pub trait ResultDistribution {
    type Probability: Sized;
    type Expectation: Sized;

    fn win_prob(&self) -> Self::Probability;
    fn draw_prob(&self) -> Self::Probability;
    fn loss_prob(&self) -> Self::Probability;
    fn expected_result(&self) -> Self::Expectation;
}

/// This is the return type of an evaluator that computes a policy over the available moves.
pub trait Policy<G>
where
    G: GameState,
{
    type Probability;

    fn policy(&self) -> Vec<(G::Move, Self::Probability)>;
}

/// This trait is used to evaluate the strength of a player's position on the board. It can do
/// things like compute a Q value for Q-learning, a policy, or really any kind of useful
/// information that can be used to make decisions in the game e.g. in AlphaZero, this would
/// compute both Q for the reachable positions and a preliminary policy for these positions too.
/// The actual MCTS should be handled by a struct that implements Strategy<G, E>.
pub trait Evaluator<G: GameState> {
    type Evaluation;

    fn new() -> Self;
    fn evaluate(&self, state: &G, mov: &G::Move) -> Self::Evaluation;
}

/// Useful struct for when you do not need to evaluate how advantageous a position is e.g. you are
/// playing randomly.
#[derive(Debug)]
pub struct EmptyEvaluator;
impl<G> Evaluator<G> for EmptyEvaluator
where
    G: GameState,
{
    type Evaluation = ();

    fn new() -> Self {
        EmptyEvaluator
    }

    fn evaluate(&self, _state: &G, _mov: &G::Move) -> Self::Evaluation {
        ()
    }
}
