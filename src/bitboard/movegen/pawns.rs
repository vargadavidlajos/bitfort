use super::*;

impl Board {

  pub fn add_pawn_quiets(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset: u8 = self.side_to_move * 6;
    let mut pawns: u64 = self.bitboards[offset as usize];
    while pawns != 0 {
      let next_sq = pop_lsb(&mut pawns);

      let mut quiets: u64 = self.get_pseudo_pawn_moves(next_sq) & move_mask;
      quiets = self.get_pin_masked_moves(quiets, next_sq);
      while quiets != 0 {
        let to_sq = quiets.trailing_zeros();

        if (self.side_to_move == 0 && quiets.trailing_zeros() / 8 == 7)
        || (self.side_to_move == 1 && quiets.trailing_zeros() / 8 == 0) {
          for piece_type in [4, 3, 2, 1] {
            buffer.add(BitMove::quiet(
              next_sq,
              to_sq,
              true,
              piece_type
            ));
          }
        }
        else {
          buffer.add(BitMove::quiet(
            next_sq,
            to_sq,
            false,
            1
          ));
        }
        quiets &= !(1 << to_sq);
      }
    }
  }
  pub fn add_pawn_captures(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset = 6 * self.side_to_move as usize;
    let mut pawns: u64 = self.bitboards[offset];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while pawns != 0 {
      let next_sq = pop_lsb(&mut pawns);
      let mut attacks: u64 = self.get_pseudo_pawn_captures(next_sq) & move_mask;
      let pin_mask = self.get_pin_mask(next_sq);

      if self.en_passant_square != 0
        && (attacks & self.en_passant_square & pin_mask != 0) {
        let king = self.bitboards[5 + offset];
        let to_sq = self.en_passant_square.trailing_zeros();
        let captured_sq = if self.side_to_move == 0 {(to_sq - 8) as usize} else {(to_sq + 8) as usize};
        let threat_mask = BISHOP_MOVE_MASK[captured_sq] | RAY_TABLE[captured_sq][0] | RAY_TABLE[captured_sq][4];
        if king & threat_mask == 0 {
          buffer.add(BitMove::en_passant(
            next_sq,
            to_sq 
          ));
        }
        else {
          let king_sq = king.trailing_zeros();
          if king & BISHOP_MOVE_MASK[captured_sq] != 0 {
            let sliders = self.bitboards[8 - offset] | self.bitboards[10 - offset];
            if !self.is_en_passant_pinned(king_sq, captured_sq as u32, sliders) {
              buffer.add(BitMove::en_passant(
                next_sq,
                to_sq 
              ));
            }
          }
          else {
            let sliders = self.bitboards[9 - offset] | self.bitboards[10 - offset];
            if !self.is_en_passand_pinned_horizontal(next_sq, captured_sq as u32, king_sq, sliders) {
              buffer.add(BitMove::en_passant(
                next_sq,
                to_sq 
              ));
            }
          }
        }
      }

      attacks &= pin_mask;
      attacks &= opponents;
      while attacks != 0 {
        let to_sq = attacks.trailing_zeros();

        if (self.side_to_move == 0 && attacks.trailing_zeros() / 8 == 7)
        || (self.side_to_move == 1 && attacks.trailing_zeros() / 8 == 0) {
          for piece_type in [4, 3, 2, 1] {
            buffer.add(BitMove::capture(
              next_sq,
              to_sq,
              true,
              piece_type
            ));
          }
        }
        else {
          buffer.add(BitMove::capture(
            next_sq,
            to_sq,
            false,
            1
          ));
        }
        attacks &= !(1 << to_sq);
      }
    }
  }
  pub fn add_pawn_moves(&self, capture_buffer: &mut MoveBuffer, quiet_buffer: &mut MoveBuffer, move_mask: u64) {
    self.add_pawn_captures(capture_buffer, move_mask);
    self.add_pawn_quiets(quiet_buffer, move_mask);
  }
}