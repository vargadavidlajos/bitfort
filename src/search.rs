pub mod searchcontext;

use std::io::{self, Error};

use crate::bitboard::movebuffer::MoveBuffer;

use searchcontext::SearchContext;


pub const MAX_DEPTH: usize = 21;

pub struct Engine {
  search_buffers: [MoveBuffer; MAX_DEPTH],
  temp_buffer: MoveBuffer,
  search_depth: u8
}

impl Engine {
  pub fn default() -> Self {
    let default_buffer = MoveBuffer::new();
    return Self {
      search_buffers: [default_buffer; MAX_DEPTH],
      temp_buffer: default_buffer,
      search_depth: 10
    };
  }
  pub fn new(s_depth: u8) -> Result<Self, Error> {
    let default_buffer = MoveBuffer::new();
    if s_depth < 2 {
      return Err(Error::new(io::ErrorKind::InvalidInput, "search depth has to be at least 2 plies!"));
    }
    return Ok(Self {
      search_buffers: [default_buffer; MAX_DEPTH],
      temp_buffer: default_buffer,
      search_depth: s_depth
    });
  }
}