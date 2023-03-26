use std::collections::{BTreeMap, HashSet};
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

///Create the edge table for scoring (sidenote, pain)
pub fn edge_table() -> BTreeMap<i32, HashSet<usize>>{
    let mut table = BTreeMap::new();
    table.insert(4, HashSet::from_iter(vec![0, 7, 56, 63]));
    table.insert(2, HashSet::from_iter(vec![2, 3, 4, 5, 16, 23, 24, 31, 32, 39, 40, 47, 58, 59, 60, 61]));
    table.insert(1, HashSet::from_iter(vec![18, 21, 27, 28, 35, 36, 42, 45]));
    table.insert(0, HashSet::from_iter(vec![19, 20, 26, 29, 34, 37, 43, 44]));
    table.insert(-1, HashSet::from_iter(vec![10, 11, 12, 13, 17, 22, 25, 30, 33, 38, 41, 46, 50, 51, 52, 53]));
    table.insert(-3, HashSet::from_iter(vec![1, 6, 8, 15, 48, 55, 57, 62]));
    table.insert(-4, HashSet::from_iter(vec![9, 14, 49, 54]));
    table
}