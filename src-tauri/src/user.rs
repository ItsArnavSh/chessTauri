//This section will be called a lot of times in the multiplayer
//This section will directly reply to the clicks coming from the frontend
use crate::helper;
use crate::moves;
use serde::Serialize;
use serde_json::json;
#[derive(Serialize)]
struct GameData {
    pieces: Option<Vec<Vec<String>>>,
    moves: Option<Vec<Vec<String>>>,
}


pub fn clicked(id:&str)->String
{
    let click = helper::coord_to_bitboard(id);
    let game_board = helper::read_file();
    let prevclick = (&game_board)[16];
    if click&prevclick != 0
    {
        println!("Yea this can move");
        //move_piece();
    }
    let friend = game_board[(&game_board)[15] as usize & 1];
    let game_data = GameData {
        pieces = None,
        moves: moves::generate_moves(click),
    };
    let serialized_game_data = serde_json::to_string(&game_data).unwrap();
    serialized_game_data
}


pub fn start_game()-> String
{
    let pieces = create_and_save_pieces();

    let game_data = GameData {
        pieces,
        moves: None,
    };

    let serialized_game_data = serde_json::to_string(&game_data).unwrap();
    println!("{}",serialized_game_data);
    serialized_game_data
}
fn create_and_save_pieces() -> Vec<Vec<std::string::String>>
{
    let bit_board:Vec<u64> = vec![
    0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000, //All pieces of Black
    0b00010000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,//All pieces of white

    0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black king
    0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Queen
    0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Rook
    0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Bishop
    0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,//Black Knight
    0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,//Black Pawn

    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,//White King
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,//White Queen
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,//White Rook
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,//White Bishop
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,//White Knight
    0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,//White Pawn

    0b0_1111_1,//abcdef
    //a means en passant available bcde for castle avaible or not, f for current turn
    0,//For en passant squares
    0//For availabel moves for the user
    ];
    helper::modify_file(&bit_board);
    return bitboard_to_pieces(&bit_board);
}
fn bitboard_to_pieces(bitboard: &Vec<u64>) -> Vec<Vec<String>> {
    let mut pieces = vec![vec![String::from("null"); 8]; 8];

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