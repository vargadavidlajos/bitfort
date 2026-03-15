use super::*;

impl Board {
  #[inline(always)]
  pub(in super) fn eval_by_material(&self) -> i32 {
    let eval = (self.bitboards[0].count_ones() as i32 - self.bitboards[6].count_ones() as i32) * PIECE_VALUE[0]
                  + (self.bitboards[1].count_ones() as i32 - self.bitboards[7].count_ones() as i32) * PIECE_VALUE[1]
                  + (self.bitboards[2].count_ones() as i32 - self.bitboards[8].count_ones() as i32) * PIECE_VALUE[2]
                  + (self.bitboards[3].count_ones() as i32 - self.bitboards[9].count_ones() as i32) * PIECE_VALUE[3]
                  + (self.bitboards[4].count_ones() as i32 - self.bitboards[10].count_ones() as i32) * PIECE_VALUE[4];
    return eval;
  }
}