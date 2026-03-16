use crate::bitboard::bitmove::BitMove;

#[derive(Copy, Clone)]
pub struct TTEntry {
  key: u64,
  best_move: BitMove,
  data: u32
}

// data: 0b0000_000000000000000000000_00000_00
//         |--| |-------------------| |---| ||
//         gen  score                 depth type
// bits:  0-1  -> entry type (0 -> null, 1 -> exact, 2 -> upperbound, 3 -> lowerbound)
// bits:  2-6  -> depth (0-31 plies searched after)
// bits:  7-27 -> score (evaluated score of the position)
// bits: 28-31 -> generation (how many searches ago was it saved)

impl TTEntry {

  pub const NULL: u8 = 0;
  pub const EXACT: u8 = 1;
  pub const UPPER: u8 = 2;
  pub const LOWER: u8 = 3;

  pub fn null() -> Self {
    return Self {
      key: 0u64,
      best_move: BitMove::null(),
      data: 0u32
    };
  }
  #[inline(always)]
  pub fn new(key: u64, best_move: BitMove, entry_type: u8, depth: u8, score: i32, generation: u8) -> Self {
    let data = (generation as u32) << 28 | (score as u32) << 7 | (depth as u32) << 2 | (entry_type as u32);
    return Self {
      key: key,
      best_move: best_move,
      data: data
    }
  }

  #[inline(always)]
  pub fn key(&self) -> u64 {
    return self.key;
  }
  #[inline(always)]
  pub fn best_move(&self) -> BitMove {
    return self.best_move.clone();
  }
  #[inline(always)]
  pub fn entry_type(&self) -> u8 {
    return (self.data & 0b11) as u8;
  }
  #[inline(always)]
  pub fn depth(&self) -> u8 {
    return ((self.data >> 2) & 0b11111) as u8;
  }
  #[inline(always)]
  pub fn score(&self) -> i32 {
    return ((self.data << 4) as i32) >> 11;
  }
  #[inline(always)]
  pub fn generation(&self) -> u8 {
    return (self.data >> 28) as u8;
  }

  #[inline(always)]
  pub fn replacement_score(&self, current_generation: u8) -> i32 {
    let age_penalty = (current_generation.wrapping_sub(self.generation()) & 0xF) as i32;
    return self.depth() as i32 * 2 - age_penalty;
  }
}