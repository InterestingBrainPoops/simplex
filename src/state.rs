#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    // Pawns, [black, white]
    piece_bit_boards: [u64; 6],
    color_bit_boards: [u64; 2],
    en_passant: Option<u64>,
    castling_rights: [bool; 4],
    turn: bool,
}

static CASTLES: [(u64, u64); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)]; // TODO: put in the proper constants for king and rook end positions
static KINGPOS: [u64; 2] = [0, 0]; // TODO: put in the proper constants for king intial positions
static ROOKPOS: [u64; 4] = [0, 0, 0, 0]; // TODO: put in the proper constants for king intial positions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Move {
    move_type: MoveType,            // The type of move.
    en_passant_square: Option<u64>, // Target en passant square.
    castling_rights: [bool; 4],     // Castling rights for both sides. KQkq is the order.
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
    // Castle type.
    BlackQueen,
    BlackKing,
    WhiteKing,
    WhiteQueen,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MoveType {
    Capture(Piece, u64, Piece, u64), // Piece that is capturing, Position of piece, Piece that is being captured, End placement
    Castle(Castle),                  // Castle Type
    Push(Piece, u64, u64, Option<u64>), //Piece type, Original position of piece, new position of piece,  new or not en passant target square.
    Promote(u64, Piece, u64), // Intial position of pawn, Piece to promote too, Final position of piece.
}
impl State {
    pub fn new(fen: &str) -> Self {
        todo!();
    }
    pub fn make_move(&mut self, move_to_make: Move) {
        self.en_passant = None;
        match move_to_make.move_type {
            MoveType::Capture(a, b, c, d) => {
                self.piece_bit_boards[a as usize] ^= b ^ d; // Move the piece
                self.piece_bit_boards[c as usize] ^= d; // remove the other piece
                self.color_bit_boards[self.turn as usize] ^= b ^ d;
                self.color_bit_boards[!self.turn as usize] ^= d;
            }
            MoveType::Castle(a) => {
                self.piece_bit_boards[Piece::King as usize] ^=
                    CASTLES[a as usize].0 ^ KINGPOS[self.turn as usize];
                self.piece_bit_boards[Piece::Rook as usize] ^=
                    CASTLES[a as usize].1 ^ ROOKPOS[a as usize];
                self.color_bit_boards[self.turn as usize] ^= CASTLES[a as usize].0
                    ^ KINGPOS[self.turn as usize]
                    ^ CASTLES[a as usize].1
                    ^ ROOKPOS[a as usize];
            }
            MoveType::Push(a, b, c, d) => {
                self.en_passant = d;
                self.piece_bit_boards[a as usize] ^= b ^ c;
                self.color_bit_boards[self.turn as usize] ^= b ^ c;
            }
            MoveType::Promote(a, b, c) => {
                self.piece_bit_boards[Piece::Pawn as usize] ^= a;
                self.piece_bit_boards[b as usize] ^= c;
                self.color_bit_boards[self.turn as usize] ^= a ^ c;
            }
        }
        self.turn = !self.turn;
    }

    pub fn unmake_move(&mut self, move_to_make: Move) {
        self.en_passant = move_to_make.en_passant_square;
        self.turn = !self.turn;
        match move_to_make.move_type {
            MoveType::Capture(a, b, c, d) => {
                self.piece_bit_boards[a as usize] ^= b ^ d; // Move the piece
                self.piece_bit_boards[c as usize] ^= d; // add the other piece
                self.color_bit_boards[self.turn as usize] ^= b ^ d;
                self.color_bit_boards[!self.turn as usize] ^= d;
            }
            MoveType::Castle(a) => {
                self.piece_bit_boards[Piece::King as usize] ^=
                    CASTLES[a as usize].0 ^ KINGPOS[self.turn as usize]; // Moves the king back
                self.piece_bit_boards[Piece::Rook as usize] ^=
                    CASTLES[a as usize].1 ^ ROOKPOS[a as usize]; // Moves the rook back
                self.color_bit_boards[self.turn as usize] ^= CASTLES[a as usize].0 // Moves the rook and king back in the color bit board.
                    ^ KINGPOS[self.turn as usize]
                    ^ CASTLES[a as usize].1
                    ^ ROOKPOS[a as usize];
            }
            MoveType::Push(a, b, c, _) => {
                self.piece_bit_boards[a as usize] ^= b ^ c; // Moves the piece back.
                self.color_bit_boards[self.turn as usize] ^= b ^ c;
            }
            MoveType::Promote(a, b, c) => {
                self.piece_bit_boards[Piece::Pawn as usize] ^= a;
                self.piece_bit_boards[b as usize] ^= c;
                self.color_bit_boards[self.turn as usize] ^= a ^ c;
            }
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        todo!();
    }
    pub fn is_in_check(&self) -> bool {
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
