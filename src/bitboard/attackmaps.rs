use once_cell::sync::Lazy;

const A_FILE: u64 = 0x0101_0101_0101_0101;
const H_FILE: u64 = 0x8080_8080_8080_8080;

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