use super::bitmove::BitMove;
use super::board::Board;

#[derive(Copy, Clone)]
pub struct MoveBuffer {

  buffer: [(BitMove, u16); 256],
  count: usize
}

impl MoveBuffer {

  pub fn new() -> Self {
    let default: BitMove = BitMove::null();
    return Self {
      buffer: [(default, 0); 256],
      count: 0
    };
  }

  #[inline]
  pub fn add(&mut self, new_move: BitMove) {
    self.buffer[self.count] = (new_move, 0);
    self.count += 1;
  }
  #[inline]
  pub fn append(&mut self, other: &MoveBuffer) {
    self.buffer[self.count..self.count + other.count()].copy_from_slice(other.contents());
    self.count += other.count();
  }
  #[inline(always)]
  pub fn clear(&mut self) {
    self.count = 0;
  }
  #[inline(always)]
  pub fn count(&self) -> usize{
    return self.count;
  }
  #[inline(always)]
  pub fn get(&self, idx: usize) -> &BitMove {
    return &self.buffer[idx].0;
  }
  #[inline(always)]
  pub fn contents(&self) -> &[(BitMove, u16)] {
    return &self.buffer[0..self.count];
  }

  #[inline(always)]
  pub fn score_moves(&mut self, board: &Board, tt_move: &BitMove, killers: (Option<BitMove>, Option<BitMove>)) {
    for i in 0..self.count() {
      let score = self.buffer[i].0.get_score(board, tt_move, killers);
      self.buffer[i].1 = score;
    }
  }
  #[inline(always)]
  pub fn order_moves(&mut self) {
    self.buffer[0..self.count].sort_unstable_by(|a, b| b.1.cmp(&a.1));
  }
}