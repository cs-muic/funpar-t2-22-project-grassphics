use rayon::iter::*;

///flatten from a 2D-board to a 1D-board
pub fn flatten_board(board: &Vec<Vec<Option<bool>>>) -> Vec<Option<bool>>{
    board.clone().into_iter().flat_map(|f| f).collect()
}

///legal moves in 1D board
pub fn is_legal_flat(x: usize, y: usize, board: &Vec<Option<bool>>, player: bool) -> bool {
    match board[x*8 + y]{
        Some(_v) => false,
        None => {
            for row in 0..3{
                for col in 0..3{
                    if !(row == 1 && col == 1) && flippable_flat(x, y, &board, player, false, row, col) { return true }
                }
            }
            return false;
        },
    }
}

///whether the board has a legal move in 1D
#[allow(dead_code)]
pub fn has_legal_flat(board: &Vec<Option<bool>>, player: bool) -> bool{
    for row in 0..8{
        for col in 0..8{
            if is_legal_flat(row, col, board, player) { return true }
        }
    }
    false
}

///get all possible legal spots to place a chip 1D board
pub fn all_legal_flat(board: &Vec<Option<bool>>, player: bool) -> Vec<usize>{
    let mut moves = Vec::new();
    for i in 0..64{
        if is_legal_flat(i/8, i%8, board, player) { moves.push(i); }
    }
    moves
}

///whether a move causes a flip in 1D
pub fn flippable_flat(x: usize, y: usize, board: &Vec<Option<bool>>, player: bool, pinged: bool, row: usize, col: usize) -> bool {
    if (col + y) < 1 || (col + y) > 8 || (row + x) < 1 || (row + x) > 8 { return false }
    match board[(x+row-1)*8 + y+col-1]{
        None => false,
        Some(v) => {
            if pinged && (player != v) { true }
            else if (player == v) && !pinged { false }
            else { flippable_flat(x + row-1, y+col-1, board, v, true, row, col) }
        }
    }
}

///check who wins in 1D, returns a value of white chips and black chips respectively in a tuple of (i32, i32)
#[allow(dead_code)]
pub fn count_winnings_flat(board: &Vec<Option<bool>>) -> (i32, i32){
    board.par_iter().fold(
        ||(0, 0), |(whi, bla), item| { match item{
            None => (whi, bla),
            Some(v) => {if *v { (whi + 1, bla) } else {(whi, bla+1)}}
        }})
        .reduce(||(0, 0), |(whi, bla), (otherwhi, otherbla)| 
        (whi + otherwhi, bla + otherbla))
}

///places a chip in a 1D board, returns a new board with the placed chip and modified board
pub fn place_chip_flat(x: usize, y: usize, board: &Vec<Option<bool>>, player: bool) -> Vec<Option<bool>>{ //please make this function handle 1D input
    let mut board_cpy = board.clone();
    for row in 0..3{
        for col in 0..3{
            if !(row == 1 && col == 1) && flippable_flat(x, y, &board, player, false, row, col) { 
                let (mut xn, mut yn) = (x, y);
                board_cpy[xn*8 + yn] = Some(player);
                while board_cpy[(xn+row-1)*8 + yn+col-1].unwrap() != player {
                    board_cpy[(xn+row-1)*8 + yn+col-1] = Some(player);
                    xn += row;
                    xn -= 1;
                    yn += col;
                    yn -= 1;
                }
             }
        }
    }
    board_cpy
}