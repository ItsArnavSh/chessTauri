//This rust module is the first one
//This will return the list of moves for any given piece
mod helper;

fn king(click:u64, friend:u64)-> u64
{
    //Watch this
    /*
    11100000
    10100000
    11100000
    */
    //This is called a cool algorithm
    //We find the distance between this bitboard shown above and our piece
    //Then drag it there, like a lookup blueprint
    let mut king_moves:u128 = 0b111000001010000011100000;//the same pattern
    //We need to shift the king to the bitboard
    king_moves=king_moves<<(helper::map_to_num(click)-15u8);
    let mut king_moves:u64 = assist::board_bound_check(king_moves) as u64;
    king_moves&=!friend;
    king_moves = assist::mask(king_moves);
    return king_moves;
}

fn knight(click:u64, friend:u64)-> u64
{
    //Watch this
    /*
    01010000
    10001000
    00o00000
    10001000
    01010000
    */
    let mut knight_moves:u128 = 0b0101000010001000000000001000100001010000;
    //It is u128 to avoid overflow
    //We will shift the knight coord
    knight_moves=knight_moves<<(helper::map_to_num(click)-22u8);
    let mut knight_moves = assist::board_bound_check(knight_moves);
    knight_moves&=!friend;//Removing friends under attack

    knight_moves = assist::mask(knight_moves);
   return knight_moves;
}

fn rook(click:u64, friend:u64, enemy:u64)-> u64
{
    let mut moves = assist::find_moves(click,1,0,enemy,friend);//Left
    moves |= assist::find_moves(click,0,1,enemy,friend);//Up
    moves |= assist::find_moves(click,-1,0,enemy,friend);//Right
    moves |= assist::find_moves(click,0,-1,enemy,friend);//Down
    moves
}

fn bishop(click:u64, friend:u64, enemy:u64)-> u64
{
    let mut moves = assist::find_moves(click,1,1,enemy,friend);
    moves |= assist::find_moves(click,-1,-1,enemy,friend);
    moves |= assist::find_moves(click,-1,1,enemy,friend);
    moves |= assist::find_moves(click,1,-1,enemy,friend);
    moves
}

fn queen(click:u64, friend:u64, enemy:u64)-> u64
{
    let moves:u64 = rook(click,friend,enemy)|bishop(click,friend,enemy);
    moves
}

mod assist
{
    pub fn mask(moves:u64)->u64
    {
        let mut moves = moves;
        //Now we will perform an antiLeak
        //Now there is an issue
        //The piece when at corner would project its pieces and they got leaked to the other end
        //As the bitboard is just one huge single dimensional array
        //To prevent this, we see which side the piece is on and clear off the other side
        //This bit notation below refers to the second half of the board
        //To see if the piece is located in the second half
        let left_leak:u64 = 0b0011111100111111001111110011111100111111001111110011;
        let right_leak:u64 = 0b1111110011111100111111001111110011111100111111001111110011111100;
        if moves & 0b1111000011110000111100001111000011110000111100001111000011110000 != 0
            {moves&=right_leak;}
        else
            {moves&=left_leak;}
        moves
    }
    pub fn board_bound_check(moves:u128)->u64
    {
        let board_bounds:u128= 0b1111111111111111111111111111111111111111111111111111111111111111;
        return (moves&board_bounds) as u64;
    }
    pub fn find_moves(moves:u64,xdirection:i8,ydirection:i8,enemy:u64,friend:u64)->u64
    {
        //Shift the board by any amount
        //Boundaries have to be checked
        /*
        11111111
        10000001
        10000001
        10000001
        10000001
        10000001
        10000001
        11111111
        */
        let mut result:u64 = 0u64;
        let mut moves = moves;
        let boundary:u64 = 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;
        loop
        {
            if xdirection>0 
            {
                moves = moves>>(xdirection as u8);
            }
            else
            {
                moves = moves<<(-xdirection as u8);
            }
            if ydirection>0
            {
                moves = moves>>8*(ydirection as u8);
            }
            else
            {
                moves = moves<<8*(-ydirection as u8);
            }
            if moves&friend != 0
            {
                break;
            }
            else if moves&enemy !=0
            {
                result |= moves;
                break;
            }
            result |= moves;
            if moves&boundary != 0
            {
                break;
            }
        }
        result
    }
}


fn main()
{}