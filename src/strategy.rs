use crate::{evaluate::*, game::*};
use rand::seq::SliceRandom;

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
