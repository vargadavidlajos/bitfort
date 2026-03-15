use super::utils::*;

#[derive(Copy, Clone)]
pub struct BitMove {
  data: u32
}
//                     15   19   23   27   31
//                11  14|  18|  22|  26|  30|
//               10| 13|| 17|| 21|| 25|| 29||
//    0123 4567 89||12|||16|||20|||24|||28|||
//  0bXXXX_XXXX_XXXX_XXXX_XXXX_XXXX_XXXX_XXXX
//
//  bits  0-1  -> move type: 0-quiet, 1-capture, 2-castling, 3-en passant
//  bits  2-7  -> from square (0-63)
//  bits  8-13 -> to square (0-63)
//  bit   14   -> promotion flag
//  bits 15-16 -> promotion piece: 0-knight, 1-bishop, 2-rook, 3-queen
//  bits 17-19 -> flags to be added...
//  bits 20-31 -> score

impl BitMove {

  pub const QUIET: u8 = 0;
  pub const CAPTURE: u8 = 1;
  pub const CASTLE: u8 = 2;
  pub const EN_PASSANT: u8 = 3;

  pub fn null() -> Self {
    Self {
      data: 0u32
    }
  }

  #[inline]
  pub fn quiet(from: u32, to: u32, is_promotion: bool, promotion_piece: u32) -> Self {

    let mut compact: u32 = 0b00_000000_000000_0_00_000_000000000000;
    compact |= (from << 24) | (to << 18) | ((is_promotion as u32) << 17) | ((promotion_piece - 1) << 15);
    
    Self {
      data: compact
    }
  }
  #[inline]
  pub fn capture(from: u32, to: u32, is_promotion: bool, promotion_piece: u32) -> Self {
    let mut compact: u32 = 0b01_000000_000000_0_00_000_000000000000;
    compact |= (from << 24) | (to << 18) | ((is_promotion as u32) << 17) | ((promotion_piece - 1) << 15);
    
    Self {
      data: compact
    }
  }
  #[inline]
  pub fn castle(from: u32, to: u32) -> Self {
    let mut compact: u32 = 0b10_000000_000000_0_00_000_000000000000;
    compact |= (from << 24) | (to << 18);
    
    Self {
      data: compact
    }
  }
  #[inline]
  pub fn en_passant(from: u32, to: u32) -> Self {
    let mut compact: u32 = 0b11_000000_000000_0_00_000_000000000000;
    compact |= (from << 24) | (to << 18);

    Self {
      data: compact
    }
  }

  #[inline(always)]
  pub fn move_type(&self) -> u8 {
    return (self.data >> 30) as u8;
  }
  #[inline(always)]
  pub fn from_square(&self) -> u8 {
    return ((self.data >> 24) & 0b111111) as u8;
  }
  #[inline(always)]
  pub fn to_square(&self) -> u8 {
    return ((self.data >> 18) & 0b111111) as u8;
  }
  #[inline(always)]
  pub fn is_promotion(&self) -> bool {
    return ((self.data >> 17) & 0b1) != 0;
  }
  #[inline(always)]
  pub fn promotion_piece(&self) -> u8 {
    return 1 + ((self.data >> 15) & 0b11) as u8;
  }

  pub fn uci_notation(&self) -> String {
    let mut notation = notation_from_square_number(self.from_square());
    notation.push_str(&notation_from_square_number(self.to_square()));

    if self.is_promotion() {
      notation.push(get_character_by_piece_id(self.promotion_piece()).to_ascii_lowercase());
    }

    return notation;
  }
}