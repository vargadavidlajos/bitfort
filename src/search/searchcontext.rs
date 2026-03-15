
pub struct SearchContext {
  pub nodes: u64,
  pub ply: usize
}

impl SearchContext {
  pub fn new() -> Self {
    return Self {
      nodes: 0,
      ply: 0
    }
  }
}