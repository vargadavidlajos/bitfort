
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