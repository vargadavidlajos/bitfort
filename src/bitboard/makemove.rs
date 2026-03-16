pub mod quiets;
pub mod captures;
pub mod castles;
pub mod enpassants;

use super::unmakeinfo::UnmakeInfo;
use super::bitmove::BitMove;
use super::board::Board;
use super::zobrist::*;

impl Board {
  #[inline]
  pub fn make_move(&mut self, played_move: &BitMove) -> UnmakeInfo {
    let move_type = played_move.move_type();

    let mut taken_piece = 0u8;
    let old_castling_rights = self.castling_rights();
    let old_en_passant_square = self.en_passant_square();
    let mut zobrist_delta = 0u64;
    let mut ep_zobrist_delta = 0u64;

    if self.en_passant_square != 0 {
      ep_zobrist_delta = ZOBRIST_EN_PASSANT[self.en_passant_square.trailing_zeros() as usize];
      self.en_passant_square = 0u64;
    }

    match move_type {
      BitMove::QUIET => {
        zobrist_delta = self.make_quiet(played_move);
      }
      BitMove::CAPTURE => {
        (taken_piece, zobrist_delta) = self.make_capture(played_move);
      }
      BitMove::CASTLE => {
        zobrist_delta = self.make_castle(played_move);
      }
      BitMove::EN_PASSANT => {
        (taken_piece, zobrist_delta) = self.make_enpassant(played_move);
      }
      _ => panic!("Tried executing move of invalid type!")
    }

    self.occupancy[2] = self.occupancy[0] | self.occupancy[1];

    self.side_to_move = 1 - self.side_to_move;

    zobrist_delta ^= ep_zobrist_delta;
    return UnmakeInfo::new(taken_piece, old_en_passant_square, old_castling_rights, zobrist_delta);
  }
}