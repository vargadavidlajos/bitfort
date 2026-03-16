pub mod transpositionentry;

use transpositionentry::TTEntry;

pub struct TranspositionTable {
  table: Vec<TTEntry>,
  generation: u8,
  hash_mask: u64,
  pub calls: u64,
  pub hits: u64
}