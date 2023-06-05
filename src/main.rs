use minimax_game::{tic_tac_toe::BoardState, EmptyEvaluator, GamePlayer, RandomStrat};
fn main() {
    let board = BoardState::new();
    let mut new_game = GamePlayer::from(board, EmptyEvaluator, RandomStrat);
    new_game.play();
}
