use once_cell::sync::Lazy;

const A_FILE: u64 = 0x0101_0101_0101_0101;
const H_FILE: u64 = 0x8080_8080_8080_8080;
const AB_FILE: u64 = 0x0303_0303_0303_0303;
const GH_FILE: u64 = 0xC0C0_C0C0_C0C0_C0C0;

// KING_ATTACK_MAP[<square_index>]
pub static KING_ATTACK_MAP: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table: [u64; 64] = [0u64; 64];

  for sq in 0..64 {
    let king: u64 = 1 << sq;

    let left_attacks: u64 = king << 7 | king >> 1 | king >> 9;
    let right_attacks: u64 = king << 1 | king << 9 | king >> 7;

    table[sq] = (left_attacks & !H_FILE) | (right_attacks & !A_FILE) | king << 8 | king >> 8;
  }
  return table;
});

// PAWN_ATTACK_MAP[<square_index>][<side>]
pub static PAWN_ATTACK_MAP: Lazy<[[u64; 2]; 64]> = Lazy::new(|| {
  let mut table: [[u64; 2]; 64] = [[0u64; 2]; 64];

  for sq in 0..64 {
    let pawn: u64 = 1 << sq;
    table[sq][0] |= (pawn << 9) & !A_FILE;
    table[sq][0] |= (pawn << 7) & !H_FILE;
  }
  for sq in 0..64 {
    let pawn: u64 = 1 << sq;
    table[sq][1] |= (pawn >> 9) & !H_FILE;
    table[sq][1] |= (pawn >> 7) & !A_FILE;
  }
  return table;
});

// KNIGHT_ATTACK_MAP[<square_index>]
pub static KNIGHT_ATTACK_MAP: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table: [u64; 64] = [0u64; 64];

  for sq in 0..64 {
    let knight: u64 = 1 << sq;

    let far_left_attacks: u64 = knight << 6 | knight >> 10;
    let near_left_attacks: u64 = knight << 15 | knight >> 17;
    let far_right_attacks: u64 = knight << 10 | knight >> 6;
    let near_right_attacks: u64 = knight << 17 | knight >> 15;

    table[sq] = (far_left_attacks & !GH_FILE) | (far_right_attacks & !AB_FILE) | (near_left_attacks & !H_FILE) | (near_right_attacks & !A_FILE);
  }
  return table;
});

// RAY_TABLE[<square_index>][<direction_index>]
pub static RAY_TABLE: Lazy<[[u64; 8]; 64]> = Lazy::new(|| {
  let mut table: [[u64; 8]; 64] = [[0u64; 8]; 64];
  let dirs: [i8; 8] = [1, 9, 8, 7, -1, -9, -8, -7];
  for sq in 0..64 {
    for d in 0..8 {
      let mut ray: u64 = 0u64;
      let origin: u64 = 1 << sq;
      let mut new_target: u64 =  if dirs[d] > 0 {origin << dirs[d]} else {origin >> -dirs[d]};
      if [0, 1, 7].contains(&d) {
        new_target &= !A_FILE;
      }
      else if [3, 4, 5].contains(&d) {
        new_target &= !H_FILE;
      }
      while new_target != 0 {
        ray |= new_target;

        new_target =  if dirs[d] > 0 {new_target << dirs[d]} else {new_target >> -dirs[d]};
        if [0, 1, 7].contains(&d) {
          new_target &= !A_FILE;
        }
        else if [3, 4, 5].contains(&d) {
          new_target &= !H_FILE;
        }
      }
      table[sq][d] = ray;
    }
  }

  return table;
});

// ROOK_MOVE_MASK[<square_index>]
pub static ROOK_MOVE_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    for dir in [0, 2, 4, 6] {
      table[sq] |= RAY_TABLE[sq][dir];
    }
  }
  table
});

// BISHOP_MOVE_MASK[<square_index>]
pub static BISHOP_MOVE_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    for dir in [1, 3, 5, 7] {
      table[sq] |= RAY_TABLE[sq][dir];
    }
  }
  table
});

// KING_SAFETY_ROOK_MASK[<square_index>]
pub static KING_SAFETY_ROOK_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    let mut mask = KING_ATTACK_MAP[sq];

    while mask != 0 {
      let next_sq = mask.trailing_zeros();
      table[sq] |= ROOK_MOVE_MASK[next_sq as usize];
      mask &= !(1 << next_sq);
    }
  }

  table
});

// KING_SAFETY_BISHOP_MASK[<square_index>]
pub static KING_SAFETY_BISHOP_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    let mut mask = KING_ATTACK_MAP[sq];

    while mask != 0 {
      let next_sq = mask.trailing_zeros();
      table[sq] |= BISHOP_MOVE_MASK[next_sq as usize];
      mask &= !(1 << next_sq);
    }
  }

  table
});