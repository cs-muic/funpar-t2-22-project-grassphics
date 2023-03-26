use crate::{rules, board_class};

pub fn get_move(board: &Vec<Vec<Option<bool>>>, player: bool) -> (usize, usize) {
    board_class::print_board(board, player);
    let mut line = String::new();
    if player {println!("\nWhite, Enter move :");}
    else {println!("\nBlack, Enter move :");}
    let _move_input = std::io::stdin().read_line(&mut line).unwrap();

    if line.len() != 4 { println!("Invalid length"); return get_move(board, player) }

    let mut line_chr = line.chars();
    let first_letter = line_chr.next().unwrap().to_ascii_lowercase();
    if !('a'..='h').contains(&first_letter ) { println!("Invalid first letter"); return get_move(board, player) }

    let second_letter = line_chr.next().unwrap().to_digit(10);
    let second_digit = match second_letter { None => {println!("Invalid second digit"); return get_move(board, player)}, Some(v) => v, };

    let col = first_letter as usize - 97;
    let row = second_digit as usize - 1;
    
    if rules::is_legal(row, col, board, player) { (first_letter as usize - 97, second_digit as usize - 1) }
    else { println!("Illegal move"); return get_move(board, player) }
}

pub fn place_chip(x: usize, y: usize, board: &Vec<Vec<Option<bool>>>, player: bool) -> Vec<Vec<Option<bool>>>{
    let mut board_cpy = board.clone();
    for row in 0..3{
        for col in 0..3{
            if !(row == 1 && col == 1) && rules::flippable(x, y, &board, player, false, row, col) { 
                let (mut xn, mut yn) = (x, y);
                board_cpy[xn][yn] = Some(player);
                while board_cpy[xn+row-1][yn+col-1].unwrap() != player {
                    board_cpy[xn+row-1][yn+col-1] = Some(player);
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