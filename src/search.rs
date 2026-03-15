pub mod searchcontext;

use crate::bitboard::movebuffer::MoveBuffer;

use searchcontext::SearchContext;


pub const MAX_DEPTH: usize = 21;

pub struct Engine {
  search_buffers: [MoveBuffer; MAX_DEPTH],
  temp_buffer: MoveBuffer,
  search_depth: u8
}