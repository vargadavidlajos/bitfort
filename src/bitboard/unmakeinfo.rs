
pub struct UnmakeInfo {
  taken_piece: u8,
  en_passant_square: u64,
  castling_rights: u8
}

impl UnmakeInfo {
  pub fn new(taken_piece: u8, en_passant: u64, castling: u8) -> Self {
    Self {
      taken_piece: taken_piece,
      en_passant_square: en_passant,
      castling_rights: castling
    }
  }

  #[inline(always)]
  pub fn taken_piece(&self) -> u8 {
    return self.taken_piece;
  }
  #[inline(always)]
  pub fn en_passant_square(&self) -> u64 {
    return self.en_passant_square;
  }
  #[inline(always)]
  pub fn castling_rights(&self) -> u8 {
    return self.castling_rights;
  }
}