use crate::{game::*, strategy::*};

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
