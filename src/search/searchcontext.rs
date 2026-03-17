use std::cmp::min;

use crate::bitboard::bitmove::BitMove;

use super::MAX_DEPTH;

pub struct SearchContext {
  pub killer1: [Option<BitMove>; MAX_DEPTH],
  pub killer2: [Option<BitMove>; MAX_DEPTH],
  hh_table: [[[u16; 64]; 64]; 2],
  pub s_nodes: u64,
  pub q_nodes: u64,
  pub ply: usize
}

impl SearchContext {
  pub fn new() -> Self {
    return Self {
      killer1: [None; MAX_DEPTH],
      killer2: [None; MAX_DEPTH],
      hh_table: [[[0; 64]; 64]; 2],
      s_nodes: 0,
      q_nodes: 0,
      ply: 0
    }
  }

  #[inline(always)]
  pub fn current_killers(&self) -> (Option<BitMove>, Option<BitMove>) {
    let ply = self.ply;
    return (self.killer1[ply], self.killer2[ply]);
  }

  #[inline]
  pub fn store_quiet_cutoff(&mut self, bitmove: BitMove, depth: u16) {
    let ply = self.ply;
    let side = ply % 2;
    let from = bitmove.from_square() as usize;
    let to = bitmove.to_square() as usize;
    let hh_score = &mut self.hh_table[side][from][to];

    if self.killer1[ply] != Some(bitmove) {
      self.killer2[ply] = self.killer1[ply];
      self.killer1[ply] = Some(bitmove);
    }

    *hh_score += depth * depth;
    *hh_score -= *hh_score >> 3;
  }

  pub fn search_end(&mut self) {
    self.killer1 = [None; MAX_DEPTH];
    self.killer2 = [None; MAX_DEPTH];
    self.hh_table = [[[0; 64]; 64]; 2];
    self.s_nodes = 0;
    self.q_nodes = 0;
    self.ply = 0;
  }

  #[inline(always)]
  pub fn hh_score(&self, bitmove: &BitMove) -> u16 {
    let side = self.ply % 2;
    let from  = bitmove.from_square() as usize;
    let to = bitmove.to_square() as usize;
    return min(500, self.hh_table[side][from][to]);
  }
}