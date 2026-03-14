

pub struct Board {
  pub(in super) bitboards: [u64; 12],     // 0-5 -> white pieces (P, N, B, R, Q, K), 6-11 -> black pieces (p, n, b, r, q, k)
  pub(in super) piece_board: [u8; 64],    // same as board indexes, 12 -> empty square
  pub(in super) occupancy: [u64; 3],      // 0 -> white, 1 -> black, 2 -> combined
  pub(in super) castling_rights: u8,      // 0b0000_KQkq
  pub(in super) pinned_squares: [u8; 64], // 0 -> E-W, 1 -> NE-SW, 2 -> N-S, 3 -> SE-NW, 4 -> no pin
  pub(in super) pin_mask: u64,            // 1 -> pin, 0 -> no pin
  pub(in super) en_passant_square: u64,   // 1 -> ep square, 0 -> no ep square
  pub(in super) side_to_move: u8          // 0 -> white to play, 1 -> black to play
}