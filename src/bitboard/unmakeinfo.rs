
pub struct UnmakeInfo {
  taken_piece: u8,
  en_passant_square: u64,
  castling_rights: u8,
  zobrist_delta: u64
}

impl UnmakeInfo {
  pub fn new(taken_piece: u8, en_passant: u64, castling: u8, zobrist: u64) -> Self {
    Self {
      taken_piece: taken_piece,
      en_passant_square: en_passant,
      castling_rights: castling,
      zobrist_delta: zobrist
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
  #[inline(always)]
  pub fn zobrist_delta(&self) -> u64 {
    return self.zobrist_delta;
  }
}