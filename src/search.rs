pub mod searchcontext;
pub mod transpositiontable;

use std::io::{self, Error};
use std::cmp::max;

use crate::bitboard::movebuffer::MoveBuffer;
use crate::bitboard::bitmove::BitMove;
use crate::bitboard::board::Board;

use transpositiontable::transpositionentry::TTEntry;
use transpositiontable::TranspositionTable;
use searchcontext::SearchContext;


pub const MAX_DEPTH: usize = 21;
pub const MAX_QDEPTH: usize = 60;

pub const MIN_MOVE_ORDER_DEPTH: usize = 2;

pub const MIN_TT_STORE_DEPTH: usize = 2;
pub const MIN_TT_LOOKUP_DEPTH: usize = 3;

pub const FUTILITY_MARGIN: i32 = 300;

pub struct Engine {
  search_buffers: [MoveBuffer; MAX_DEPTH],
  quiescence_buffers: [MoveBuffer; MAX_QDEPTH],
  temp_buffer: MoveBuffer,
  tt: TranspositionTable,
  search_depth: u8,
  quiescence_depth: u8
}

impl Engine {
  pub fn default() -> Self {
    let default_buffer = MoveBuffer::new();
    return Self {
      search_buffers: [default_buffer; MAX_DEPTH],
      quiescence_buffers: [default_buffer; MAX_QDEPTH],
      temp_buffer: default_buffer,
      tt: TranspositionTable::new(20),
      search_depth: 10,
      quiescence_depth: 4
    };
  }
  pub fn new(s_depth: u8, q_depth: u8, hash_length: u32) -> Result<Self, Error> {
    let default_buffer = MoveBuffer::new();
    if s_depth < 2 {
      return Err(Error::new(io::ErrorKind::InvalidInput, "search depth has to be at least 2 plies!"));
    }
    return Ok(Self {
      search_buffers: [default_buffer; MAX_DEPTH],
      quiescence_buffers: [default_buffer; MAX_QDEPTH],
      temp_buffer: default_buffer,
      tt: TranspositionTable::new(hash_length),
      search_depth: s_depth,
      quiescence_depth: q_depth
    });
  }

  pub fn iterative_deepening(&mut self, mut board: Board) -> (BitMove, i32, u64, u64) {
    let mut latest = (BitMove::null(), 0, 0, 0);
    let mut ctx = SearchContext::new();
    let start_depth = if self.search_depth > 3 { 4 } else { 2 };

    let alpha = -1_000_000;
    let beta  =  1_000_000;

    for depth in start_depth..=self.search_depth as usize {

      latest = self.main_search(depth, &mut board, alpha, beta, &mut ctx);

      if latest.1.abs() > 900_000 {
        break;
      }
    }
    return latest;
  }

  fn main_search(&mut self, depth: usize, board: &mut Board, mut alpha: i32, beta: i32, ctx: &mut SearchContext) -> (BitMove, i32, u64, u64) {
    
    ctx.s_nodes += 1;

    board.collect_moves(&mut self.search_buffers[depth], &mut self.temp_buffer);
    
    let mut tt_move = BitMove::null();
    if let Some(entry) = self.tt.get(board.hash()) {
      tt_move = entry.best_move();
    }

    self.search_buffers[depth].score_moves(&board, &tt_move, ctx);
    self.search_buffers[depth].order_moves();
    
    let mut best_move = self.search_buffers[depth].get(0).clone();
    for i in 0..self.search_buffers[depth].count() {
      let bitmove = self.search_buffers[depth].get(i).clone();

      let undo = board.make_move(&bitmove);
      board.update_hash(undo.zobrist_delta());
      let eval = -self.negamax(depth - 1, board, -beta, -alpha, ctx);
      board.unmake_move(&bitmove, &undo);
      board.update_hash(undo.zobrist_delta());

      if eval > alpha {
        alpha = eval;
        best_move = bitmove;
      }
    }
    self.tt.store_exact(board.hash(), best_move, depth as u8, alpha);

    return if board.side_to_move() == 0 { (best_move, alpha, ctx.s_nodes, ctx.q_nodes) } else { (best_move, -alpha, ctx.s_nodes, ctx.q_nodes) }; 
  }

  fn negamax(&mut self, depth: usize, board: &mut Board, mut alpha: i32, beta: i32, ctx: &mut SearchContext) -> i32 {
    
    ctx.s_nodes += 1;

    if depth == 0 {
      let (has_moves, in_check) = board.has_moves();

      if !has_moves {
        if in_check {
          return -999_999 + (ctx.ply as i32)/2;
        }
        else { return 0; }
      }
      return self.quiescence(0, board, alpha, beta, &mut ctx.q_nodes)
    }

    let is_in_check = board.collect_moves(&mut self.search_buffers[depth], &mut self.temp_buffer);

    if self.search_buffers[depth].count() == 0 {

      if is_in_check {
        return -999_999 + (ctx.ply as i32)/2;
      }
      else {
        return 0;
      }
    }

    let mut tt_move = BitMove::null();
    let mut found_entry: Option<TTEntry> = None;
    if depth >= MIN_TT_LOOKUP_DEPTH {
      found_entry = self.tt.get(board.hash());
      if let Some(entry) = found_entry {
        tt_move = entry.best_move();
      }
    }

    ctx.ply += 1;
    if depth >= MIN_MOVE_ORDER_DEPTH {
      self.search_buffers[depth].score_moves(board, &tt_move, ctx);
      self.search_buffers[depth].order_moves();
    }

    let mut raised_alpha = false;
    let mut best_move = self.search_buffers[depth].get(0).clone();
    let mut best_score = i32::MIN + 1;

    let undo = board.make_move(&best_move);
    board.update_hash(undo.zobrist_delta());
    let eval = -self.negamax(depth - 1, board, -beta, -alpha, ctx);
    board.unmake_move(&best_move, &undo);
    board.update_hash(undo.zobrist_delta());

    if eval > alpha {
      alpha = eval;
      raised_alpha = true;
    }
    else if !raised_alpha {
      best_score = max(best_score, eval);
    }

    if alpha >= beta {
      if depth >= MIN_TT_STORE_DEPTH {
        self.tt.store_lower(board.hash(), best_move, depth as u8, alpha);
      }
      if best_move.move_type() == BitMove::QUIET {
        ctx.store_quiet_cutoff(best_move, depth as u16);
      }

      ctx.ply -= 1;
      return alpha;
    }

    if let Some(entry) = found_entry {
      if entry.depth() as usize >= depth {
        match entry.entry_type() {
          TTEntry::UPPER => {
            if entry.score() <= alpha {
              ctx.ply -= 1;
              return entry.score()
            }
          }
          TTEntry::LOWER => {
            if entry.score() >= beta {
              ctx.ply -= 1;
              return entry.score();
            }
            alpha = max(alpha, entry.score());
          }
          TTEntry::EXACT => {
            ctx.ply -= 1;
            return entry.score();
          }
          entry_type => { panic!("incorrect entry type returned: {}", entry_type) }
        };
      }
    }

    for i in 1..self.search_buffers[depth].count() {

      let bitmove = self.search_buffers[depth].get(i).clone();
      
      let undo = board.make_move(&bitmove);
      board.update_hash(undo.zobrist_delta());
      let eval = -self.negamax(depth - 1, board, -beta, -alpha, ctx);
      board.unmake_move(&bitmove, &undo);
      board.update_hash(undo.zobrist_delta());

      if eval > alpha {
        alpha = eval;
        raised_alpha = true;
        best_move = bitmove;
      }
      else if !raised_alpha {
        best_score = max(best_score, eval);
      }

      if alpha >= beta {
        if depth >= MIN_TT_STORE_DEPTH {
          self.tt.store_lower(board.hash(), best_move, depth as u8, alpha);
        }
        if bitmove.move_type() == BitMove::QUIET {
          ctx.store_quiet_cutoff(bitmove, depth as u16);
        }

        ctx.ply -= 1;
        return alpha;
      }
    }
    ctx.ply -= 1;

    if depth >= MIN_TT_STORE_DEPTH {
      if raised_alpha {
        self.tt.store_exact(board.hash(), best_move, depth as u8, alpha);
      }
      else {
        self.tt.store_upper(board.hash(), best_move, depth as u8, best_score);
      }
    }

    return alpha; 
  }

  fn quiescence(&mut self, ply: usize, board: &mut Board, mut alpha: i32, beta: i32, nodes: &mut u64) -> i32 {
    
    *nodes += 1;

    if ply >= self.quiescence_depth as usize {
      let eval = board.evaluation();
      return if board.side_to_move() == 0 { eval } else { -eval };
    }

    let in_check = board.collect_captures(&mut self.quiescence_buffers[ply], &mut self.temp_buffer);

    if self.quiescence_buffers[ply].count() == 0 {
      if in_check {
        return -999_999 + (ply as i32)/2;
      }
      let eval = board.evaluation();
      return if board.side_to_move() == 0 { eval } else { -eval };
    }

    if ply < 4 {
      let stat_eval = if board.side_to_move() == 0 { board.futility_evaluation() } else { -board.futility_evaluation() };
      if stat_eval >= beta {
        return stat_eval;
      }
      if !in_check && stat_eval + FUTILITY_MARGIN < alpha {
        return alpha;
      } 
      alpha = max(stat_eval, alpha);
    }

    self.quiescence_buffers[ply].q_score_moves(board);
    self.quiescence_buffers[ply].order_moves();

    for i in 0..self.quiescence_buffers[ply].count() {

      let bitmove = self.quiescence_buffers[ply].get(i).clone();

      if !in_check && bitmove.skip_see(board) {
        continue;
      }

      let undo = board.make_move(&bitmove);
      let eval = -self.quiescence(ply + 1, board, -beta, -alpha, nodes);
      board.unmake_move(&bitmove, &undo);

      alpha = max(alpha, eval);

      if alpha > beta {
        break;
      }
    }

    return alpha; 
  }
}