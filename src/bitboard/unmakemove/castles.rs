use super::*;

impl Board {
  pub fn unmake_castle(&mut self, played_move: &BitMove, undo_info: &UnmakeInfo) {
    let main_from = played_move.from_square() as usize;
    let main_to = played_move.to_square() as usize;

    let color_offset = 6 - 6 * self.side_to_move;
    let friendly_occupancy = 1 - self.side_to_move as usize;

    let main_piece = 5 + color_offset as usize;
    let secondary_piece: usize = main_piece - 2;
    let is_kingside = main_to%8 > 4;
    let secondary_from: usize = if is_kingside { main_to + 1 } else { main_to - 2 };
    let secondary_to: usize = if is_kingside { main_to - 1 } else { main_to + 1 };

    self.bitboards[main_piece] &= !(1 << main_to);
    self.occupancy[friendly_occupancy] &= !(1 << main_to);
    self.piece_board[main_to] = Self::EMPTY_SQUARE;

    self.bitboards[main_piece] |= 1 << main_from;
    self.occupancy[friendly_occupancy] |= 1 << main_from;
    self.piece_board[main_from] = main_piece as u8;

    self.bitboards[secondary_piece] &= !(1 << secondary_to);
    self.occupancy[friendly_occupancy] &= !(1 << secondary_to);
    self.piece_board[secondary_to] = Self::EMPTY_SQUARE;

    self.bitboards[secondary_piece] |= 1 << secondary_from;
    self.occupancy[friendly_occupancy] |= 1 << secondary_from;
    self.piece_board[secondary_from] = secondary_piece as u8;
  }
}