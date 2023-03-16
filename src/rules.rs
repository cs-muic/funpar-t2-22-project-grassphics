use rayon::iter::*;

pub fn is_legal(x: usize, y: usize, board: &Vec<Vec<Option<bool>>>, player: bool) -> bool {
    match board[x][y]{
        Some(_v) => false,
        None => {
            for row in 0..3{
                for col in 0..3{
                    if !(row == 1 && col == 1) && flippable(x, y, &board, player, false, row, col) { return true }
                }
            }
            return false;
        },
    }
}

pub fn has_legal(board: &Vec<Vec<Option<bool>>>, player: bool) -> bool{
    for row in 0..8{
        for col in 0..8{
            if is_legal(row, col, board, player) { return true }
        }
    }
    false
}

pub fn flippable(x: usize, y: usize, board: &Vec<Vec<Option<bool>>>, player: bool, pinged: bool, row: usize, col: usize) -> bool {
    if (col + y) < 1 || (col + y) > 8 || (row + x) < 1 || (row + x) > 8 { return false }
    match board[x+row-1][y+col-1]{
        None => false,
        Some(v) => {
            if pinged && (player != v) { true }
            else if (player == v) && !pinged { false }
            else { flippable(x + row-1, y+col-1, board, v, true, row, col) }
        }
    }
}

pub fn count_winnings(board: &Vec<Vec<Option<bool>>>) -> (i32, i32){
    board.par_iter().fold(
        ||(0, 0), |base, row| 
        row.par_iter().fold(
            ||(0, 0), |(whi, bla), item| { match item{
            None => base,
            Some(v) => {if *v { (whi + 1, bla) } else {(whi, bla+1)}}
        }})
        .reduce(||(0, 0), |(whi, bla), (otherwhi, otherbla)| 
        (whi + otherwhi, bla + otherbla)))
    .reduce(||(0, 0), |(whi, bla), (otherwhi, otherbla)| 
    (whi + otherwhi, bla + otherbla))
}