pub mod luts;
pub mod material;
pub mod pst;
pub mod pawnstructure;

use super::board::Board;
use luts::*;
use super::utils::pop_lsb;

impl Board {
  #[inline(always)]
  pub fn evaluation(&self) -> i32 {
    return self.eval_by_material()
         + self.eval_by_pst()
         + self.pawn_structure();
  }

  #[inline(always)]
  pub fn futility_evaluation(&self) -> i32 {
    return self.eval_by_material()
          + self.eval_by_pst();
  }
}