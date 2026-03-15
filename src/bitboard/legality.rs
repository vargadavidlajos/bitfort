use super::board::Board;
use super::attackmaps::RAY_TABLE;
use super::checkinfo::CheckInfo;
use super::attacks::get_raycast_from_square_in_direction;
use super::utils::*;

impl Board {
  pub fn check_test(&self) -> CheckInfo {
    let mut check_info: CheckInfo = CheckInfo::new();
    let offset: usize = 6 * self.side_to_move as usize;
    let king: u64 = self.bitboards[5 + offset];
    let king_sq = king.trailing_zeros() as usize;
    let occupancy = self.occupancy[2];

    // queen-rook checks (+)
    let attacker_mask = self.bitboards[10 - offset] | self.bitboards[9 - offset];

    for dir in [0, 2, 4, 6] {
      let threat_mask: u64 = get_raycast_from_square_in_direction(occupancy, king_sq, dir);
      if threat_mask & attacker_mask != 0 {
        check_info.add_checker(threat_mask);
      }
    }

    // queen-bishop checks (x)
    let attacker_mask = self.bitboards[10 - offset] | self.bitboards[8 - offset];

    for dir in [1, 3, 5, 7] {
      let threat_mask = get_raycast_from_square_in_direction(occupancy, king_sq, dir);
      if threat_mask & attacker_mask != 0 {
        check_info.add_checker(threat_mask);
      }
    }

    // knight checks (L)
    let attacker_mask = self.bitboards[7 - offset];
    let threat_mask = self.get_pseudo_knight_moves(king_sq as u32);
    let checker = threat_mask & attacker_mask;
    if checker != 0 {
      check_info.add_checker(checker);
    }

    // pawn checks (v)
    let attacker_mask = self.bitboards[6 - offset];
    let threat_mask = self.get_pseudo_pawn_captures(king_sq as u32);
    let checker = threat_mask & attacker_mask;
    if checker != 0 {
      check_info.add_checker(checker);
    }

    return check_info;
  }

  pub(in super) fn calc_pinned_squares(&mut self) {
    self.pinned_squares = [4; 64];
    self.pin_mask = 0u64;

    let friendly_pieces: u64 = self.occupancy[self.side_to_move as usize];
    let offset: usize = 6 * self.side_to_move as usize;
    let king_board: u64 = self.bitboards[5 + offset];
    let king_sq: u32 = king_board.trailing_zeros();
    let opponent_queen_bishop_mask: u64 = self.bitboards[8 - offset] | self.bitboards[10 - offset];
    let opponent_queen_rook_mask: u64 = self.bitboards[9 - offset] | self.bitboards[10 - offset];

    // Queen-Rook directions
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 0);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 2);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 4);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 6);

    // Queen-Bishop directions
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 1);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 3);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 5);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 7);
  }

  pub(in super) fn set_pinned_in_ray_direction(&mut self, king_sq: u32, friendly_pieces: u64, attackers: u64, dir: u8) {
    let is_up: bool = dir / 4 == 0;
    let mask: u64 = RAY_TABLE[king_sq as usize][dir as usize];
    let blockers: u64 = self.occupancy[2] & mask;
    if blockers == 0 { return; }
    let first_blocker_sq: u32 = if is_up { blockers.trailing_zeros() } else { 63 - blockers.leading_zeros() };
    if (friendly_pieces & 1 << first_blocker_sq) != 0 {
      let blockers: u64 = blockers & !(1 << first_blocker_sq);
      if blockers == 0 { return; }
      let second_blocker_sq: u32 = if is_up { blockers.trailing_zeros() } else { 63 - blockers.leading_zeros() };

      if (attackers & 1 << second_blocker_sq) != 0 {
        self.pinned_squares[first_blocker_sq as usize] = dir % 4;
        self.pin_mask |= 1 << first_blocker_sq;
      }
    }
  }
  #[inline]
  pub(in super) fn get_pin_masked_moves(&self, moves: u64, sq: u32) -> u64 {
    let sq: usize = sq as usize;
    if self.pinned_squares[sq] == 4 { return moves; }
    let dir: u8 = self.pinned_squares[sq];
    return moves & (RAY_TABLE[sq][dir as usize] | RAY_TABLE[sq][4 + dir as usize]);
  }
  #[inline]
  pub fn get_pin_mask(&self, sq: u32) -> u64 {
    let sq: usize = sq as usize;
    if self.pinned_squares[sq] == 4 { return !0u64; }
    let dir: u8 = self.pinned_squares[sq];
    return RAY_TABLE[sq][dir as usize] | RAY_TABLE[sq][4 + dir as usize];
  }

  pub(in super) fn is_en_passant_pinned(&self, king_sq: u32, pawn_sq: u32, sliders: u64) -> bool {
    let occupancy = self.occupancy[2];
    let pawn: u64 = 1 << pawn_sq;
    let ray = RAY_TABLE[king_sq as usize][1];
    if pawn & ray != 0 {
      let mut blockers = ray & occupancy;
      let first_blocker = pop_lsb(&mut blockers);
      if first_blocker != pawn_sq 
      || blockers == 0 {
        return false;
      }
      let second_blocker = blockers.trailing_zeros();
      return sliders & 1 << second_blocker != 0;
    }

    let ray = RAY_TABLE[king_sq as usize][3];
    if pawn & ray != 0 {
      let mut blockers = ray & occupancy;
      let first_blocker = pop_lsb(&mut blockers);
      if first_blocker != pawn_sq 
      || blockers == 0 {
        return false;
      }
      let second_blocker = blockers.trailing_zeros();
      return sliders & 1 << second_blocker != 0;
    }
    
    let ray = RAY_TABLE[king_sq as usize][5];
    if pawn & ray != 0 {
      let mut blockers = ray & occupancy;
      let first_blocker = pop_msb(&mut blockers);
      if first_blocker != pawn_sq 
      || blockers == 0 {
        return false;
      }
      let second_blocker = 63 - blockers.leading_zeros();
      return sliders & 1 << second_blocker != 0;
    }

    let ray = RAY_TABLE[king_sq as usize][7];
    if pawn & ray != 0 {
      let mut blockers = ray & occupancy;
      let first_blocker = pop_msb(&mut blockers);
      if first_blocker != pawn_sq 
      || blockers == 0 {
        return false;
      }
      let second_blocker = 63 - blockers.leading_zeros();
      return sliders & 1 << second_blocker != 0;
    }

    panic!("en passant pin to king not found!");
  }
  pub(in super) fn is_en_passand_pinned_horizontal(&self, pawn_sq: u32, captured_sq: u32, king_sq: u32, sliders: u64) -> bool {
    
    if sliders == 0 { return false; }
    let captured_pawn_mask: u64 = !(1 << captured_sq);
    let king: u64 = 1 << king_sq;
    let search_mask = king | sliders;
    let occupancy = self.occupancy[2];

    let right_blockers = RAY_TABLE[pawn_sq as usize][0] & captured_pawn_mask & occupancy;
    let left_blockers = RAY_TABLE[pawn_sq as usize][4] & captured_pawn_mask & occupancy;
    if right_blockers & search_mask == 0 
    || left_blockers & search_mask == 0 {
      return false;
    }
    
    let first_right_blocker = 1 << right_blockers.trailing_zeros();
    let first_left_blocker = 1 << (63 - left_blockers.leading_zeros());
    return (king & first_left_blocker != 0 && sliders & first_right_blocker != 0)
        || (king & first_right_blocker != 0 && sliders & first_left_blocker != 0);
  }

  pub fn has_moves(&mut self) -> (bool, bool) {
    self.calc_pinned_squares();
    let check_info = self.check_test();

    let has_moves =  match check_info.check_count {
      0 => self.has_any_moves(),
      1 => self.has_any_moves_in_check(check_info.move_mask),
      2 => self.has_any_moves_in_double_check(),
      _ => panic!("too many checkers")
    };
    return (has_moves, check_info.check_count > 0);
  }
  pub(in super) fn has_any_moves(&self) -> bool {
    let offset = 6*self.side_to_move as usize;

    let mut moves = self.get_bulk_pseudo_pawn_pushes(self.bitboards[offset]);
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_pawn_attacks(self.bitboards[offset]);
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_knight_moves(self.bitboards[1 + offset]);
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_bishop_moves(self.bitboards[2 + offset]);
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_rook_moves(self.bitboards[3 + offset]);
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_queen_moves(self.bitboards[4 + offset]);
    if moves != 0 { return true; }

    let safe_squares = self.get_safe_king_squares();
    moves = self.get_pseudo_king_moves(self.bitboards[5 + offset].trailing_zeros()) & safe_squares & !self.occupancy[2];
    if moves != 0 { return true; }

    return false;
  }
  pub(in super) fn has_any_moves_in_check(&self, block_mask: u64) -> bool {
    let offset = 6 * self.side_to_move as usize;

    let mut moves = self.get_bulk_pseudo_pawn_pushes(self.bitboards[offset]) & block_mask;
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_pawn_attacks(self.bitboards[offset]) & block_mask;
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_knight_moves(self.bitboards[1 + offset]) & block_mask;
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_bishop_moves(self.bitboards[2 + offset]) & block_mask;
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_rook_moves(self.bitboards[3 + offset]) & block_mask;
    if moves != 0 { return true; }

    moves = self.get_bulk_pseudo_queen_moves(self.bitboards[4 + offset]) & block_mask;
    if moves != 0 { return true; }

    let safe_squares = self.get_safe_king_squares();
    moves = self.get_pseudo_king_moves(self.bitboards[5 + offset].trailing_zeros()) & safe_squares & !self.occupancy[self.side_to_move as usize];
    if moves != 0 { return true; }
    
    return false;
  }
  pub(in super) fn has_any_moves_in_double_check(&self) -> bool {
    let offset = 6 * self.side_to_move as usize;

    let safe_squares = self.get_safe_king_squares();
    let moves = self.get_pseudo_king_moves(self.bitboards[5 + offset].trailing_zeros()) & safe_squares & !self.occupancy[2];
    if moves != 0 { return true; }

    return false;
  }
}