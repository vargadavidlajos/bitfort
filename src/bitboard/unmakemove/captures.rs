use super::*;

impl Board {
  pub fn unmake_capture(&mut self, played_move: &BitMove, undo_info: &UnmakeInfo) {
    let main_from = played_move.from_square() as usize;
    let main_to = played_move.to_square() as usize;

    let color_offset = 6 - 6 * self.side_to_move;
    let friendly_occupancy = 1 - self.side_to_move as usize;

    let secondary_piece: usize = undo_info.taken_piece() as usize;
    let secondary_from: usize = main_to as usize;

    let opponent_occupancy = secondary_piece/6;
      
    if played_move.is_promotion() {
      let main_piece = color_offset as usize;
      let promotion_piece = (color_offset + played_move.promotion_piece()) as usize;
          
      self.bitboards[main_piece] |= 1 << main_from;
      self.occupancy[friendly_occupancy] |= 1 << main_from;
      self.piece_board[main_from] = main_piece as u8;

      self.bitboards[promotion_piece] &= !(1 << main_to);
      self.occupancy[friendly_occupancy] &= !(1 << main_to);
      self.piece_board[main_to] = Self::EMPTY_SQUARE;
    }
    else {
      let main_piece = self.piece_board(main_to as u8) as usize;
          
      self.bitboards[main_piece] |= 1 << main_from;
      self.occupancy[friendly_occupancy] |= 1 << main_from;
      self.piece_board[main_from] = main_piece as u8;

      self.bitboards[main_piece] &= !(1 << main_to);
      self.occupancy[friendly_occupancy] &= !(1 << main_to);
      self.piece_board[main_to] = Self::EMPTY_SQUARE;
    }

    self.bitboards[secondary_piece] |= 1 << secondary_from;
    self.occupancy[opponent_occupancy] |= 1 << secondary_from;
    self.piece_board[secondary_from] = secondary_piece as u8;
  }
}