use crate::bitboard::bitmove::BitMove;

use super::MAX_DEPTH;

pub struct SearchContext {
  pub killer1: [Option<BitMove>; MAX_DEPTH],
  pub killer2: [Option<BitMove>; MAX_DEPTH],
  pub nodes: u64,
  pub ply: usize
}

impl SearchContext {
  pub fn new() -> Self {
    return Self {
      killer1: [None; MAX_DEPTH],
      killer2: [None; MAX_DEPTH],
      nodes: 0,
      ply: 0
    }
  }

  #[inline(always)]
  pub fn current_killers(&self) -> (Option<BitMove>, Option<BitMove>) {
    let ply = self.ply;
    return (self.killer1[ply], self.killer2[ply]);
  }
}