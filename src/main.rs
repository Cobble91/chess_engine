use chess::ps_tables;
use crate::Piece::*;

fn main() {
    println!("Hello, world!");
}

const BLACK: bool = false;
const WHITE: bool = true;
const INIT_BOARD:Board = Board{
    pieces: [
        WRook, WKnight, WBishop, WQueen, WKing, WBishop, WKnight, WRook,
        WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn,
        BRook, BKnight, BBishop, BQueen, BKing, BBishop, BKnight, BRook,
    ],
    stm: WHITE, w_qs_castle: true, w_ks_castle: true, b_qs_castle: true, b_ks_castle: true, en_passant: 0,
};

// struct for storing the game state
#[derive(Debug, Clone, Copy)]
struct Board {
    pieces: [Piece; 64], // 0 = A1, 1 = A2, ..., 7 = A8, 8 = B1, ..., 63 = H8

    stm: bool, // side to move

    w_qs_castle: bool, // castling rights
    w_ks_castle: bool,
    b_qs_castle: bool,
    b_ks_castle: bool,

    en_passant: u64, // bitboard for en passant targets, 1 = targetable
}

#[derive(Debug, Clone, Copy)]
enum Piece { // actual value of enums = piece value, used for easier scoring
    Empty = 0,
    WPawn = 100,
    WRook = 500,
    WKnight = 320,
    WBishop = 350,
    WQueen = 900,
    WKing = 400,
    BPawn = -100,
    BRook = -500,
    BKnight = -320,
    BBishop = -350,
    BQueen = -900,
    BKing = -400,
}

fn static_eval(board: Board) -> i32 {
    let mut material_score: i32 = 0;
    let mg_score = 0; // score difference weighted by mg ps tables
    let eg_score = 0; // ^ by eg ps tables
    let material = 0; // total score of all pieces unweighted, used to check mg vs eg
    for i from 0 to 63 { // TODO: fix
        let value = p as i32;
        let ps_tables: [[i32; 64]; 2] = match p {
            Empty => ps_tables::PAWN_TABLES,
            WPawn | BPawn => ps_tables::PAWN_TABLES,
            WRook | BRook => ps_tables::ROOK_TABLES,
            WKnight | BKnight => ps_tables::KNIGHT_TABLES,
            WBishop | BBishop => ps_tables::BISHOP_TABLES,
            WQueen | BQueen => ps_tables::QUEEN_TABLES,
            WKing | BKing => ps_tables::KING_TABLES,
        };
        mg_score += value + ps_tables[0][i]; // TODO: make pstable count for white but against blk
        eg_score += value + ps_tables[1][i];
    }
    0
}
