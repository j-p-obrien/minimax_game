use minimax_game::{evaluate::*, game::*, strategy::*, tic_tac_toe::BoardState};
fn main() {
    let board = BoardState::new();
    let mut new_game = GamePlayer::from(board, EmptyEvaluator, RandomStrat);
    new_game.play();
}
