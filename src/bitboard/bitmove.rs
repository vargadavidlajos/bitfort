
pub struct BitMove {
  data: u32
}
//                     15   19   23   27   31
//                11  14|  18|  22|  26|  30|
//               10| 13|| 17|| 21|| 25|| 29||
//    0123 4567 89||12|||16|||20|||24|||28|||
//  0bXXXX_XXXX_XXXX_XXXX_XXXX_XXXX_XXXX_XXXX
//
//  bits  0-1  -> move type: 0-quiet, 1-capture, 2-castling, 3-en passant
//  bits  2-7  -> from square (0-63)
//  bits  8-13 -> to square (0-63)
//  bit   14   -> promotion flag
//  bits 15-16 -> promotion piece: 0-knight, 1-bishop, 2-rook, 3-queen
//  bits 17-19 -> flags to be added...
//  bits 20-31 -> score