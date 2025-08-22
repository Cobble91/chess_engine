use chess::ps_tables;
use crate::Piece::*;

fn main() {
    println!("Chess.");
    let b:Board = INIT_BOARD;
    let eval = static_eval(b);
    println!("Initial Board Eval: {eval}\n\n");
    let b = Board{
        pieces: [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            WPawn, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
        ],
    stm: WHITE, w_qs_castle: true, w_ks_castle: true, b_qs_castle: true, b_ks_castle: true, en_passant: 0,
    };
    let eval = static_eval(b);
    println!("Test Board Eval: {eval}");
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

    en_passant: u64, // bitboard for en passant targets, 1 = targetable // TODO: change away from
                     // bitboard
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

fn static_eval(board: Board) -> i32 { // currently returs positive = white winning & vice versa. to
                                      // change this, use stm field in board struct
    // let mut material_score: i32 = 0; // i dont remember why i made this var
    let mut mg_score = 0; // score difference weighted by mg ps tables
    let mut eg_score = 0; // ^ by eg ps tables
    let mut material = 0; // total score of all pieces unweighted, used to check mg vs eg
    
    for i in 0..=63 {
        let piece = board.pieces[i];
        let value = piece as i32; // numerical value of current piece
        if value == 0 { // skip calculations for empty square
            continue;
        }
        
        // find relevant piece square tables, both middle and end game
        let ps_tables: [[i32; 64]; 2] = match piece {
            Empty => ps_tables::PAWN_TABLES,
            WPawn | BPawn => ps_tables::PAWN_TABLES,
            WRook | BRook => ps_tables::ROOK_TABLES,
            WKnight | BKnight => ps_tables::KNIGHT_TABLES,
            WBishop | BBishop => ps_tables::BISHOP_TABLES,
            WQueen | BQueen => ps_tables::QUEEN_TABLES,
            WKing | BKing => ps_tables::KING_TABLES,
        };
        let mg_table = ps_tables[0];
        let eg_table = ps_tables[1];

        println!("Index: {i}");
        println!("Piece: {piece:?}");
        println!("Value: {value}");
        print!("Color: ");

        mg_score += value;
        eg_score += value;
        if value < 0 { // if black, invert ps value
            println!("black");
            println!("PSVal: {}", mg_table[i]);
            mg_score -= mg_table[i];
            eg_score -= eg_table[i];
        }
        else { // if white, flip ps_table
               // because ps_tables are stored with current player at the bottom(i = 48 to 63) and
               // opponent at the top(i = 0 to 15) (opposite of how the board struct stores it)
            let i_flipped = 56 - (i / 8)*8 + (i % 8); // flip the position vertically
            println!("white, Flipped Index: {i_flipped}");
            println!("PSVal: {}", mg_table[i_flipped]);
            mg_score += mg_table[i_flipped];
            eg_score += eg_table[i_flipped];
        }
        material += value;
        println!();
    }
    mg_score
}
