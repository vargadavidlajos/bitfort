pub mod pawns;
pub mod knights;
pub mod bishops;
pub mod rooks;
pub mod queens;
pub mod kings;

use super::board::Board;
use super::movebuffer::MoveBuffer;
use super::bitmove::BitMove;
use super::utils::*;

impl Board {

  const NO_FILTER: u64 = 0xFFFF_FFFF_FFFF_FFFF;

  pub fn collect_all_moves(&self, buffer: &mut MoveBuffer, temp_buffer: &mut MoveBuffer) {
    let safe_squares = self.get_safe_king_squares();

    self.add_pawn_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_knight_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_bishop_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_rook_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_queen_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_king_moves(buffer, temp_buffer, safe_squares);
    self.add_king_castles(buffer, safe_squares);

    buffer.append(temp_buffer);
    temp_buffer.clear();
  }
}