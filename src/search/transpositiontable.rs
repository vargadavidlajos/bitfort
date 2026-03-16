pub mod transpositionentry;

use transpositionentry::TTEntry;

pub struct TranspositionTable {
  table: Vec<TTEntry>,
  generation: u8,
  hash_mask: u64,
  pub calls: u64,
  pub hits: u64
}

impl TranspositionTable {

  pub fn new(hash_length: u32) -> Self {
    let capacity = 2u64.pow(hash_length) as usize;
    let default_entry = TTEntry::null();
    return Self {
      table: vec![default_entry; capacity],
      generation: 0,
      hash_mask: !0u64 >> (64 - hash_length),
      calls: 0,
      hits: 0
    };
  }

  #[inline(always)]
  pub fn new_generation(&mut self) {
    self.generation = (self.generation + 1)%16;
  }

  #[inline(always)]
  pub fn get(&mut self, hash: u64) -> Option<TTEntry> {
    let index = (hash & self.hash_mask) as usize;
    let entry = self.table[index];

    if entry.entry_type() != TTEntry::NULL
      && entry.key() == hash {
        return Some(entry);
    }
    return None;
  }
}