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
pub fn print_board(board: &Vec<Vec<Option<bool>>>){
    for row in board{
        for item in row{ match item {
            None => print!("X "),
            Some(v) => {
                if *v { print!("■ "); }
                else { print!("□ "); }
            }
        } 
        }
        println!();
    }
}