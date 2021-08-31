#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pawns: [u64; 2],
    bishops: [u64; 2],
    knights: [u64; 2],
    kings: [u64; 2],
    queens: [u64; 2],
    rooks: [u64; 2],
    en_passant: Option<u64>,
    castling_rights: [bool; 4],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Move {
    move_type: MoveType,
    en_passant_square: Option<u64>,
    castling_rights: [bool; 4],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Piece {
    Rook,
    King,
    Queen,
    Bishop,
    Pawn,
    Knight,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Castle {
    BlackQueen,
    BlackKing,
    WhiteKing,
    WhiteQueen,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MoveType {
    Capture(Piece, u64, Piece, u64),
    Castle(Castle),
    Push(Piece, u64, Option<u64>),
}
impl State {
    pub fn new(fen: &str) -> Self {
        todo!();
    }
    pub fn make_move(&mut self, move_to_make: Move) {
        todo!();
    }
    pub fn get_moves(&self) -> Vec<Move> {
        todo!();
    }
    pub fn is_in_check(&self) -> bool {
        todo!();
    }
    pub fn unmake_move(&mut self, move_to_make: Move) {
        todo!();
    }
    pub fn perft(&mut self, depth: u8) -> u64 {
        let mut nodes = 0;
        for x in self.get_moves() {
            self.make_move(x);
            if !self.is_in_check() {
                nodes += self.perft(depth - 1);
            }
            self.unmake_move(x);
        }
        nodes
    }
}
