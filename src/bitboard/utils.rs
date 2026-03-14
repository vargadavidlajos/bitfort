
const RANK_NUMBERS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub fn notation_from_square_number(sq: u8) -> String {
  let row = sq / 8;
  let col = sq % 8;
  let mut notation = String::new();

  let row_not = RANK_NUMBERS[row as usize];
  let col_not = FILE_LETTERS[col as usize];

  notation.push(col_not);
  notation.push(row_not);
  return notation;
}

pub fn try_get_square_number_from_notation(notation: &str) -> Result<u8, ()> {

  let file = match notation.chars().nth(0).unwrap() {
    'a' => 0,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
     _  => { return Result::Err(()); }
  };
  if let Some(rank) = notation.chars().nth(1) {
    return Result::Ok(file + 8 * (rank.to_digit(10).unwrap() as u8) - 8);
  }
  else {
    return Result::Err(());
  }
}