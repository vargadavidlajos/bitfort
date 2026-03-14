use super::bitmove::BitMove;

pub struct MoveBuffer {

  buffer: [BitMove; 256],
  count: usize
}

impl MoveBuffer {

  pub fn new() -> Self {
    return Self {
      buffer: [BitMove::quiet(0, 0, false, 1); 256],
      count: 0
    };
  }
}