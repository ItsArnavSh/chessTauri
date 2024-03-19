use std::fs::File;
use std::io::prelude::*;
pub fn map_to_num(bitmap:u64)->u8
{
    let mut bitmap = bitmap;
    for i in 1..65
    {
        if (bitmap&1)==1
                {return i;}
            bitmap=bitmap>>1;
    }
    0
}
pub fn coord_to_bitboard(coord: &str) -> u64 {
    let file = coord.chars().nth(0).unwrap() as u8 - 'a' as u8;
    let rank = coord.chars().nth(1).unwrap() as u8 - '1' as u8;
    1 << (rank * 8 + file)
}
pub fn modify_file(bit_board:&Vec<u64>)
{
    let serialized_bitboard = serde_json::to_string(&bit_board).unwrap();

    // Write to a file
    let mut file = File::create("bitboard.json").expect("Could not create file");
    file.write_all(serialized_bitboard.as_bytes()).expect("Could not write to file");
}
pub fn read_file()->Vec<u64>
{
    let serialized_bitboard = std::fs::read_to_string("bitboard.json").expect("Could not read file");
    serde_json::from_str(&serialized_bitboard).unwrap()
}
