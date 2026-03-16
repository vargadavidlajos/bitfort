use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub static ZOBRIST_TABLE: Lazy<[[u64; 12]; 64]> = Lazy::new(|| {
  let mut rng = StdRng::seed_from_u64(0xCAFFEECAB);
  let mut table = [[0u64; 12]; 64];

  for sq in 0..64 {
    for b in 0..12 {
      table[sq][b] = rng.r#gen();
    }
  }
  table
});
pub static ZOBRIST_EN_PASSANT: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut rng: StdRng = StdRng::seed_from_u64(0xCABCAFFEE);
  let mut table: [u64; 64] = [0u64; 64];

  for sq in 0..64 {
    table[sq] = rng.r#gen();
  }
  table
});

// ZOBRIST_EXTRAS[0] -> black queenside castle
// ZOBRIST_EXTRAS[1] -> black kingside castle
// ZOBRIST_EXTRAS[2] -> white queenside castle
// ZOBRIST_EXTRAS[3] -> white kingside castle
// ZOBRIST_EXTRAS[4] -> side to move
pub static ZOBRIST_EXTRAS: Lazy<[u64; 5]> = Lazy::new(|| {
  let mut rng: StdRng = StdRng::seed_from_u64(0xCCAABFFEE);
  let mut table: [u64; 5] = [0u64; 5];

  for i in 0..5 {
    table[i] = rng.r#gen();
  }
  table
});