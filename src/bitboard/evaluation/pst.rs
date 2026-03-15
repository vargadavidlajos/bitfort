use super::*;

impl Board {
  #[inline]
  fn eval_by_pst(&self) -> i32 {
    let eval = (Self::pawn_pst(self.bitboards[0], true) - Self::pawn_pst(self.bitboards[6], false))
                  + (Self::knight_pst(self.bitboards[1], true) - Self::knight_pst(self.bitboards[7], false))
                  + (Self::bishop_pst(self.bitboards[2], true) - Self::bishop_pst(self.bitboards[8], false))
                  + (Self::rook_pst(self.bitboards[3], true) - Self::rook_pst(self.bitboards[9], false))
                  + (Self::queen_pst(self.bitboards[4], true) - Self::queen_pst(self.bitboards[10], false))
                  + (Self::king_pst(self.bitboards[5], true) - Self::king_pst(self.bitboards[11], false));
    return eval;
  }
  #[inline]
  fn pawn_pst(board: u64, is_white: bool) -> i32 {
    let mut score  = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += PAWN_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += PAWN_PST[sq];
      }
    }
    return score as i32;
  }
  #[inline]
  fn knight_pst(board: u64, is_white: bool) -> i32 {
    let mut score = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += KNIGHT_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += KNIGHT_PST[sq];
      }
    }
    return score as i32;
  }
  #[inline]
  fn bishop_pst(board: u64, is_white: bool) -> i32 {
    let mut score = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += BISHOP_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += BISHOP_PST[sq];
      }
    }
    return score as i32;
  }
  #[inline]
  fn rook_pst(board: u64, is_white: bool) -> i32 {
    let mut score = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += ROOK_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += ROOK_PST[sq];
      }
    }
    return score as i32;
  }
  #[inline]
  fn queen_pst(board: u64, is_white: bool) -> i32 {
    let mut score = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += QUEEN_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += QUEEN_PST[sq];
      }
    }
    return score as i32;
  }
  #[inline]
  fn king_pst(board: u64, is_white: bool) -> i32 {
    let mut score = 0;
    let mut board = board;
    if is_white {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += KING_PST[63 - sq];
      }
    }
    else {
      while board != 0 {
        let sq = board.trailing_zeros() as usize;
        board &= !(1 << sq);

        score += KING_PST[sq];
      }
    }
    return score as i32;
  }
}