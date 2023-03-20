use crate::rules;
use colored::Colorize;

//create a board where true -> white; false -> black
pub fn create_board() -> Vec<Vec<Option<bool>>>{
    let mut board = Vec::new();
    for i in 0..8 {
        let mut row = Vec::new();
        for l in 0..8 {
            if (i == 3 && l == 4) || (i == 4 && l == 3) { row.push(Some(false)); }
            else if (i == 3 && l == 3) || (i == 4 && l == 4) { row.push(Some(true)); }
            else {row.push(None)}
        }
        board.push(row);
    }
    board
}

//print the entire board
pub fn print_board(board: &Vec<Vec<Option<bool>>>, player: bool){
    for row in 0..8{
        for item in 0..8{ match board[row][item] {
            None => {
                if rules::is_legal(row, item, board, player){ print!("{}", "O ".red()); }
                else {print!("{}","■ ".green());}
            },
            Some(v) => {
                if v { print!("■ "); }
                else { print!("□ "); }
            }
        } 
        }
        println!();
    }
}