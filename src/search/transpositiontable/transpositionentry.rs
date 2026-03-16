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
