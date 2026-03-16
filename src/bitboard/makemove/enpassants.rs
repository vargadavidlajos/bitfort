use super::*;

impl Board {
  pub fn make_enpassant(&mut self, played_move: &BitMove) -> (u8, u64) {
    let main_from: usize = played_move.from_square() as usize;
    let main_to: usize = played_move.to_square() as usize;
    let main_piece: usize = self.piece_board(main_from as u8) as usize;
    let friendly_occupancy = main_piece/6;

    let color_offset = self.side_to_move as usize;
    let secondary_piece = 6 - 6*color_offset;
    let secondary_from = main_to - 8 + 16*color_offset;

    let mut zobrist_delta = 0u64;

    let opponent_occupancy = 1 - self.side_to_move as usize;
        
    self.bitboards[main_piece] &= !(1 << main_from);
    zobrist_delta ^= ZOBRIST_TABLE[main_from][main_piece];
    self.occupancy[friendly_occupancy] &= !(1 << main_from);
    self.piece_board[main_from] = Self::EMPTY_SQUARE;

    self.bitboards[secondary_piece] &= !(1 << secondary_from);
    zobrist_delta ^= ZOBRIST_TABLE[secondary_from][secondary_piece];
    self.occupancy[opponent_occupancy] &= !(1 << secondary_from);
    self.piece_board[secondary_from] = Self::EMPTY_SQUARE;
      
    self.bitboards[main_piece] |= 1 << main_to;
    zobrist_delta ^= ZOBRIST_TABLE[main_to][main_piece];
    self.occupancy[friendly_occupancy] |= 1 << main_to;
    self.piece_board[main_to] = main_piece as u8;

    return (secondary_piece as u8, zobrist_delta);
  }
}