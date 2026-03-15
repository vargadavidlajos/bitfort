pub mod quiets;
pub mod captures;
pub mod castles;
pub mod enpassants;

use super::bitmove::BitMove;
use super::board::Board;

impl Board {
  #[inline]
  pub fn make_move(&mut self, played_move: &BitMove) {
    let move_type = played_move.move_type();

    match move_type {
      BitMove::QUIET => {
        self.make_quiet(played_move);
      }
      BitMove::CAPTURE => {
        self.make_capture(played_move);
      }
      BitMove::CASTLE => {
        self.make_castle(played_move);
      }
      BitMove::EN_PASSANT => {
        self.make_enpassant(played_move);
      }
      _ => panic!("Tried executing move of invalid type!")
    }

    self.occupancy[2] = self.occupancy[0] | self.occupancy[1];

    if self.en_passant_square != 0 {
      self.en_passant_square = 0u64;
    }

    self.side_to_move = 1 - self.side_to_move;
  }
}