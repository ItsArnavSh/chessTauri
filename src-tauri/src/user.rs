//This section will be called a lot of times in the multiplayer
//This section will directly reply to the clicks coming from the frontend
use serde_json;

pub fn start_game()-> String
{
    let pieces = vec![
        vec!["pieces/rook-b.svg", "pieces/knight-b.svg", "pieces/bishop-b.svg", "pieces/queen-b.svg", "pieces/king-b.svg", "pieces/bishop-b.svg", "pieces/knight-b.svg", "pieces/rook-b.svg"],
        vec!["pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg", "pieces/pawn-b.svg"],
        vec!["", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "", "", "", ""],
        vec!["pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg", "pieces/pawn-w.svg"],
        vec!["pieces/rook-w.svg", "pieces/knight-w.svg", "pieces/bishop-w.svg", "pieces/queen-w.svg", "pieces/king-w.svg", "pieces/bishop-w.svg", "pieces/knight-w.svg", "pieces/rook-w.svg"]
    ];

    let pieces_json = serde_json::to_string(&pieces).unwrap();
    pieces_json
}
fn createAndSavePieces()
{
    let bitBoard:Vec<u64> = vec![
    0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000, //All pieces of Black
    0b00010000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,//All pieces of white

    0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black king
    0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Queen
    0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Rook
    0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Bishop
    0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Knight
    0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,//Black Pawn

    0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000,//White King
    0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,//White Queen
    0b00000000_00000000_00000000_00000000_00000000_00000000_10000001_00000000,//White Rook
    0b00000000_00000000_00000000_00000000_00000000_00000000_00100100_00000000,//White Bishop
    0b00000000_00000000_00000000_00000000_00000000_00000000_01000010_00000000,//White Knight
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111,//White Pawn

    0b0_1111_1,//abcdef
    //a means en passant available bcde for castle avaible or not, f for current turn

    0,//For en passant squares
    0//For availabel moves for the user
    ];
    bitboard_to_pieces(bitBoard)
}
fn bitboard_to_pieces(bitboard: &[u64; 16]) -> Vec<Vec<String>> {
    let mut pieces = vec![vec![String::from(""); 8]; 8];

    for i in 0..64 {
        let row = 7 - (i / 8);
        let col = i % 8;

        if (bitboard[0] >> i) & 1 == 1 {
            // Black piece
            if (bitboard[2] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/king-b.svg".to_string();
            } else if (bitboard[3] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/queen-b.svg".to_string();
            } else if (bitboard[4] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/rook-b.svg".to_string();
            } else if (bitboard[5] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/bishop-b.svg".to_string();
            } else if (bitboard[6] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/knight-b.svg".to_string();
            } else if (bitboard[7] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/pawn-b.svg".to_string();
            }
        } else if (bitboard[1] >> i) & 1 == 1 {
            // White piece
            if (bitboard[8] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/king-w.svg".to_string();
            } else if (bitboard[9] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/queen-w.svg".to_string();
            } else if (bitboard[10] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/rook-w.svg".to_string();
            } else if (bitboard[11] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/bishop-w.svg".to_string();
            } else if (bitboard[12] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/knight-w.svg".to_string();
            } else if (bitboard[13] >> i) & 1 == 1 {
                pieces[row][col] = "pieces/pawn-w.svg".to_string();
            }
        }
    }

    pieces
}