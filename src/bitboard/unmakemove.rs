pub mod quiets;
pub mod captures;
pub mod castles;
pub mod enpassants;

use super::board::Board;
use super::unmakeinfo::UnmakeInfo;
use super::bitmove::BitMove;

impl Board {
  #[inline]
  pub fn unmake_move(&mut self, played_move: &BitMove, undo_info: &UnmakeInfo) {
    let move_type: u8 = played_move.move_type();

    match move_type {
      BitMove::QUIET => {
        
      }
      BitMove::CAPTURE => {
        
      }
      BitMove::CASTLE => {
        
      }
      BitMove::EN_PASSANT => {
        
      }
      _ => { panic!("Tried to revert move of invalid type!"); }
    }

    self.castling_rights = undo_info.castling_rights();
    self.en_passant_square = undo_info.en_passant_square();

    self.side_to_move = 1 - self.side_to_move;

    self.occupancy[2] = self.occupancy[0] | self.occupancy[1];
  }
}