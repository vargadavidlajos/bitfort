use super::*;

impl Board {
  fn pawn_structure(&self) -> i32 {
    let white_pawn_mask = self.bitboards[0];
    let black_pawn_mask = self.bitboards[6];

    let mut white_eval = 0;
    let mut black_eval = 0;
    let mut white_pawns = white_pawn_mask;
    let mut black_pawns = black_pawn_mask;

    white_eval -= Self::doubled_pawn_penalties(white_pawn_mask); // doubled pawns
    while white_pawns != 0 {
      let pawn_sq = pop_lsb(&mut white_pawns) as usize;

      white_eval += 150 * (PASSED_PAWN_MASK[pawn_sq][0] & black_pawn_mask == 0) as i32; // passed pawn
      white_eval -=  30 * (PASSED_PAWN_MASK[pawn_sq][1] & white_pawn_mask == 0) as i32; // backward pawn
    }

    black_eval -= Self::doubled_pawn_penalties(black_pawn_mask); // doubled pawns
    while black_pawns != 0 {
      let pawn_sq = pop_lsb(&mut black_pawns) as usize;

      black_eval += 150 * (PASSED_PAWN_MASK[pawn_sq][1] & white_pawn_mask == 0) as i32; // passed pawn
      black_eval -=  30 * (PASSED_PAWN_MASK[pawn_sq][0] & black_pawn_mask == 0) as i32; // backward pawn
    }

    return white_eval - black_eval;
  }
  #[inline]
  fn doubled_pawn_penalties(pawns: u64) -> i32 {
    let mut penalty = 0;
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[0]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[1]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[2]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[3]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[4]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[5]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[6]).count_ones() as usize];
    penalty += DOUBLED_PAWN_PENALTY[(pawns & FILE_MASK[7]).count_ones() as usize];

    return penalty;
  }
}