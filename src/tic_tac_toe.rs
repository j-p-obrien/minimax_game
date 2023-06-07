use crate::game::*;
use std::fmt::Display;

/// Used to represent the pieces on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Piece {
    #[default]
    X,
    O,
    Empty,
}

/// We will encode positions using a bitboard. Position 0 is the upper left position on the board
/// and we count from left to right then top to bottom, like so:
/// 0 | 1 | 2
/// 3 | 4 | 5
/// 6 | 7 | 8
/// The rightmost logical bit of the u16 Position is position 0. Bits are 1 if occupied and 0
/// otherwise. It would make sense to make this a struct to ensure that the positions are always
/// valid, but since this is not exposed to the user I will not add the extra boilerplate
/// necessary for this.
type Position = u16;

/// Represents a move. A single 1 bit denotes which position to move to. Note that only the 9
/// rightmost logical bits may be 1, since we have only 9 squares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(Position);

/// The state of the board. player1 and player2 encode the position for Player 1 and Player 2,
/// respectively. to_move encodes which player's turn it is. player1_piece encodes whether player
/// 1 is X's or O's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BoardState {
    player1: Position,
    player2: Position,
    to_move: Player,
    player1_piece: Piece,
}

/// This encodes the winning positions. If A is the position of a player, then the player is in
/// a winning position only if (A & WINNING_POSITIONS[i]) == WINNING_POSITIONS[i] for some i.
const WINNING_POSITIONS: [Position; 8] = [
    // Top row
    0b0000_0000_0000_0111,
    // Middle row
    0b0000_0000_0011_1000,
    // Bottom row
    0b0000_0001_1100_0000,
    // Left column
    0b0000_0000_0100_1001,
    // Middle column
    0b0000_0000_1001_0010,
    // Right column
    0b0000_0001_0010_0100,
    // Upper Left Diagonal
    0b0000_0001_0001_0001,
    // Upper Right Diagonal
    0b0000_0000_0101_0100,
];

/// Array of all potential moves that can be made. These are all u16's with a single 1 bit i.e.
/// Move(2^k) represents moving to position k on the board. This could also be an iterator but I
/// think (hope?) this is faster.
const ALL_MOVES: [Move; 9] = [
    Move(1),
    Move(2),
    Move(4),
    Move(8),
    Move(16),
    Move(32),
    Move(64),
    Move(128),
    Move(256),
];

/// If all of these positions are occupied and there is no winner yet then the game is a draw.
/// If A and B are the positions of players A and B then the game is a draw only if:
/// (A | B) & DRAW == DRAW
const DRAW: Position = 0b0000_0001_1111_1111;

impl Piece {
    pub fn other(&self) -> Piece {
        match *self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            Piece::Empty => Piece::Empty,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match *self {
            Piece::X => "X",
            Piece::O => "O",
            Piece::Empty => "_",
        };
        write!(f, "{}", piece)
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0u16..3 {
            let _ = self.display_row(f, i)?;
        }
        println!();
        Ok(())
    }
}

impl BoardState {
    fn display_row(&self, f: &mut std::fmt::Formatter<'_>, row: Position) -> std::fmt::Result {
        let row_offset = row * 3;
        let mut buffer = [Piece::Empty; 3];
        buffer.iter_mut().enumerate().for_each(|(i, piece)| {
            let shift = row_offset + i as u16;
            if (self.player1 >> shift) & 1 == 1 {
                *piece = self.player1_piece;
            } else if (self.player2 >> shift) & 1 == 1 {
                *piece = self.player1_piece.other();
            }
        });
        write!(f, "{}|{}|{}\n", buffer[0], buffer[1], buffer[2])
    }

    pub fn new() -> BoardState {
        BoardState::default()
    }

    pub fn pick_piece_new(player1_piece: Piece) -> BoardState {
        BoardState {
            player1_piece,
            ..Default::default()
        }
    }

    /// Returns true if the given move is legal i.e. the desired position is unoccupied
    pub fn move_is_legal(&self, move_candidate: &Move) -> bool {
        let filled_positions = self.player1 | self.player2;
        (move_candidate.0 & filled_positions) == 0
    }

    /// Gets all the legal moves given the current board state.
    pub fn legal_moves(&self) -> Vec<Move> {
        ALL_MOVES
            .into_iter()
            .filter(|candidate| self.move_is_legal(candidate))
            .collect()
    }

    pub fn next_state(&self, mov: &Move) -> BoardState {
        let mut board_clone = self.clone();
        board_clone.apply_move(mov);
        board_clone
    }

    /// This function does not check whether a move is valid or not. The safer option is to use
    /// try_move(), which will check to see if a move is legal before doing it. This should only
    /// be used for performance reasons or if you have already checked that the move is legal.
    pub fn apply_move(&mut self, mov: &Move) {
        *self.current_player_position_mut() |= mov.0;
        self.to_move.flip_player()
    }

    /// Returns true if the player who last moved has won the game.
    pub fn last_player_is_winner(&self) -> bool {
        WINNING_POSITIONS
            .into_iter()
            .any(|pos| (pos & self.last_player_position()) == pos)
    }

    /// Returns true if the player whose turn it is to move is in a winning state. This probab
    pub fn current_player_is_winner(&self) -> bool {
        WINNING_POSITIONS
            .into_iter()
            .any(|pos| (pos & self.current_player_position()) == pos)
    }

    /// Returns true if the given Player is in a winning state.
    pub fn is_winner(&self, player: &Player) -> bool {
        let position = if *player == Player::One {
            self.player1
        } else {
            self.player2
        };
        WINNING_POSITIONS
            .into_iter()
            .any(|pos| (position & pos) == pos)
    }

    /// Returns Some(Player) if Player is in a winning position; None otherwise
    pub fn get_winner(&self) -> Option<Player> {
        if self.last_player_is_winner() {
            Some(self.last_player())
        } else if self.current_player_is_winner() {
            Some(self.current_player())
        } else {
            None
        }
    }

    /// Returns true if the game is a draw; false otherwise.
    pub fn is_draw(&self) -> bool {
        (self.player1 | self.player2) & DRAW == DRAW
    }

    /// Returns the current player.
    pub fn current_player(&self) -> Player {
        self.to_move
    }

    /// Returns the player who last moved.
    pub fn last_player(&self) -> Player {
        self.to_move.other_player()
    }

    /// Returns an immutable reference to the given player's position.
    fn get_position(&self, player: &Player) -> &Position {
        if *player == Player::One {
            &self.player1
        } else {
            &self.player2
        }
    }

    /// Returns a mutable reference to the given player's position.  
    fn get_position_mut(&mut self, player: &Player) -> &mut Position {
        if *player == Player::One {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    /// Returns an immutable reference to the current player's position.
    fn current_player_position(&self) -> &Position {
        self.get_position(&self.current_player())
    }

    /// Returns a mutable reference to the current player's position.
    fn current_player_position_mut(&mut self) -> &mut Position {
        self.get_position_mut(&self.current_player())
    }

    /// Returns an immutable reference to the position of the last player who moved.
    fn last_player_position(&self) -> &Position {
        self.get_position(&self.last_player())
    }

    #[allow(dead_code)]
    /// Returns a mutable reference to the position of the last player who moved.
    fn last_player_position_mut(&mut self) -> &mut Position {
        self.get_position_mut(&self.last_player())
    }
}

impl GameState for BoardState {
    type Move = Move;

    fn new() -> Self {
        BoardState::new()
    }

    fn legal_moves(&self) -> Vec<Self::Move> {
        self.legal_moves()
    }

    fn next_state(&self, mov: &Self::Move) -> Self {
        self.next_state(mov)
    }

    fn apply_move(&mut self, mov: &Self::Move) {
        self.apply_move(mov)
    }

    fn current_player(&self) -> Player {
        self.current_player()
    }

    fn game_result(&self) -> GameResult {
        if let Some(winner) = self.get_winner() {
            GameResult::Win(winner)
        } else if self.is_draw() {
            GameResult::Draw
        } else {
            GameResult::Undetermined
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tic_tac_toe::Move;

    use super::BoardState;

    #[test]
    fn test_move() {
        let mut board1 = BoardState::new();
        let mut board2 = BoardState::new();
        board1.apply_move(&Move(0));
        board1.apply_move(&Move(1));
        board2.apply_move(&Move(1));
        board2.apply_move(&Move(0));

        assert_eq!(board1, board2)
    }
}
