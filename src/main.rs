use minimax_game::{
    evaluate::EmptyEvaluator, game::GamePlayer, strategy::RandomStrategy, tic_tac_toe::BoardState,
};
fn main() {
    let board = BoardState::new();
    let mut new_game = GamePlayer::from(board, EmptyEvaluator, RandomStrategy);
    new_game.play();
}
