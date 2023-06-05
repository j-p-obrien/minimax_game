use crate::{evaluate::*, game::*};
use rand::seq::SliceRandom;

/// This trait is used to actually compute the move taken given the current state of the game. It
/// is intended that structs implementing this trait use information provided by the evaluator to
/// make its decision e.g. in AlphaZero, this trait would involve the computation of the MCTS
/// before making the actual move.
pub trait Strategy<G, E>
where
    G: GameState,
    E: Evaluator<G>,
{
    fn new() -> Self;
    fn choose_move(&self, state: &G) -> Option<G::Move>;
}

pub struct RandomStrategy;

impl<G, E> Strategy<G, E> for RandomStrategy
where
    G: GameState,
    G::Move: Clone,
    E: Evaluator<G>,
{
    fn new() -> Self {
        Self
    }

    fn choose_move(&self, state: &G) -> Option<G::Move> {
        let legal_moves = state.get_legal_moves();
        let random_move = legal_moves.choose(&mut rand::thread_rng());
        random_move.cloned()
    }
}

/// This struct is intended to be used when your evaluator returns a value that can be ordered from
/// least to most favorable e.g. Q-values. In this case, the evaluator should evaluate favorability
/// from the perspective of whose turn the board state says it is.
pub struct GreedyStrategy<E> {
    evaluator: E,
}

impl<G, E> Strategy<G, E> for GreedyStrategy<E>
where
    G: GameState,
    E: Evaluator<G>,
    E::Evaluation: Ord,
{
    fn new() -> Self {
        GreedyStrategy {
            evaluator: E::new(),
        }
    }

    /// Greedily chooses the best move based on the value of the evaluator returns.
    fn choose_move(&self, state: &G) -> Option<<G as GameState>::Move> {
        if let Some((_, mov)) = state
            .get_state_moves()
            .into_iter()
            .max_by_key(|(state, mov)| self.evaluator.evaluate(state, mov))
        {
            Some(mov)
        } else {
            None
        }
    }
}
