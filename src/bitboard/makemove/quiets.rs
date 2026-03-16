use super::*;

impl Board {
  pub fn make_quiet(&mut self, played_move: &BitMove) -> u64 {
    let main_from: usize = played_move.from_square() as usize;
    let main_to: usize = played_move.to_square() as usize;
    let main_piece: usize = self.piece_board(main_from as u8) as usize;
    let friendly_occupancy = main_piece/6;

    let color_offset = self.side_to_move * 6;
    let castling_offset = 2 - 2 * self.side_to_move as usize;
    let castling_rights = self.castling_rights >> castling_offset;

    let mut zobrist_delta = 0u64;

    self.bitboards[main_piece] &= !(1 << main_from);
    zobrist_delta ^= ZOBRIST_TABLE[main_from][main_piece];
    self.occupancy[friendly_occupancy] &= !(1 << main_from);
    self.piece_board[main_from] = Self::EMPTY_SQUARE;

    if played_move.is_promotion() {
      let promotion_piece = (color_offset + played_move.promotion_piece()) as usize;
      self.bitboards[promotion_piece] |= 1 << main_to;
      zobrist_delta ^= ZOBRIST_TABLE[main_to][promotion_piece];
      self.occupancy[friendly_occupancy] |= 1 << main_to;
      self.piece_board[main_to] = promotion_piece as u8;
    }
    else {
      self.bitboards[main_piece] |= 1 << main_to;
      zobrist_delta ^= ZOBRIST_TABLE[main_to][main_piece];
      self.occupancy[friendly_occupancy] |= 1 << main_to;
      self.piece_board[main_to] = main_piece as u8;

      if main_piece == 0 && (main_to - main_from) == 16 {
        let new_en_passant = main_to - 8;
        self.en_passant_square = 1 << new_en_passant;
        zobrist_delta ^= ZOBRIST_EN_PASSANT[new_en_passant];
      }
      else if main_piece == 6 && (main_from - main_to) == 16 {
        let new_en_passant = main_to + 8;
        self.en_passant_square = 1 << new_en_passant;
        zobrist_delta ^= ZOBRIST_EN_PASSANT[new_en_passant];
      }
      else if main_piece == 5 + color_offset as usize 
            && castling_rights != 0 {
        if castling_rights & 0b1 != 0 {
          self.castling_rights &= !(1 << castling_offset);
          zobrist_delta ^= ZOBRIST_EXTRAS[1 + castling_offset];
        }
        if castling_rights & 0b10 != 0 {
          self.castling_rights &= !(2 << castling_offset);
          zobrist_delta ^= ZOBRIST_EXTRAS[castling_offset];
        }
      }
      else if main_piece == 3 + color_offset as usize
            && castling_rights != 0 {
        let back_rank_offset = 56 * self.side_to_move as usize;
            
        if castling_rights & 0b10 != 0
              && main_from == 7 + back_rank_offset {
          self.castling_rights &= !(2 << castling_offset);
          zobrist_delta ^= ZOBRIST_EXTRAS[1 + castling_offset];
        }
        else if castling_rights & 0b1 != 0
              && main_from == back_rank_offset {
          self.castling_rights &= !(1 << castling_offset);
          zobrist_delta ^= ZOBRIST_EXTRAS[castling_offset];
        }
      }
    }
    return zobrist_delta;
  }
}