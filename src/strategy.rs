use crate::{evaluate::*, game::*};
use rand::seq::SliceRandom;

/// This trait is used to actually compute the move taken given the current state of the game. It
/// is intended that structs implementing this trait use information provided by the evaluator to
/// make its decision e.g. in AlphaZero, this trait would involve the computation of the MCTS
/// before making the actual move.
pub trait Strategy<G, E>
where
    G: GameState,
    E: Evaluator<G, Evaluation = Self::Evaluation>,
{
    type Evaluation;

    fn new() -> Self;
    fn choose_move(&self, state: &G, evaluator: &E) -> Option<G::Move>;
}

/*
TODO: This is broken after changes to API.
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

    fn choose_move(&self, state: &G, evaluator: &E) -> Option<G::Move> {
        let legal_moves = state.legal_moves();
        let random_move = legal_moves.choose(&mut rand::thread_rng());
        random_move.cloned()
    }
}
*/

/// This struct is intended to be used when your evaluator returns a value that can be ordered from
/// least to most favorable e.g. Q-values. In this case, the evaluator should evaluate favorability
/// from the perspective of whose turn the board state says it is.

/*
pub struct GreedyStrategy;

TODO: This is broken with latest changes to API.
impl<G, E> Strategy<G, E> for GreedyStrategy
where
    G: GameState,
    E: Evaluator<G>,
    E::Evaluation: Ord,
{



    fn new() -> Self {
        GreedyStrategy
    }

    /// Greedily chooses the best move based on the value of the evaluator returns.
    fn choose_move(&self, state: &G, evaluator: &E) -> Option<<G as GameState>::Move> {
        state
            .legal_moves()
            .into_iter()
            .max_by_key(|mov| evaluator.evaluate(state, mov))
    }
}
*/

#[derive(Debug)]
pub struct TerminalStateStrategy;

impl<G> Strategy<G, TerminalStateEvaluator> for TerminalStateStrategy
where
    G: GameState,
    G::Move: Clone,
{
    type Evaluation = GameResult;
    fn new() -> Self {
        Self
    }

    // Computes the best move and returns Some(move). If there are no moves available return None.
    fn choose_move(&self, state: &G, evaluator: &TerminalStateEvaluator) -> Option<<G>::Move> {
        let current_result = state.game_result();
        if current_result != GameResult::Undetermined {
            return None;
        }

        let current_player = state.current_player();
        let states_and_moves = state.states_and_moves();
        for (future_state, mov) in &states_and_moves {
            if future_state.game_result() == GameResult::Win(current_player) {
                // TODO: If mov is expensive to clone this is suboptimal
                return Some(mov.clone());
            }
        }

        for (future_state, mov) in &states_and_moves {
            if !future_state.game_result().is_determined() {
                if let Some(opponents_move) = self.choose_move(future_state, evaluator) {
                    //let expected_next_state = future_state.apply_move(&opponents_move)
                }
            }
        }

        todo!()
    }
}
