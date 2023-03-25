use rules::count_winnings;

mod input_move;
mod board_class;
mod rules;
mod score;
mod one_dim;
mod bot;

fn main(){
    let mut board = board_class::create_board();
    let mut player = false;
    while rules::has_legal(&board, player){ 
        if player {
            let (col,row) = bot::minimax(&board, 0); //no move count set yet
            println!("");
            board_class::print_board(&board, player);
            println!("Bot moves at: {}{}\n",((col+97) as u8) as char ,row + 1); 
            board = input_move::place_chip(row, col, &board, player);
        }
        else {
            let (col, row) = input_move::get_move(&board, player);
            //println!("{} {}",col,row); // for debugging
            board = input_move::place_chip(row, col, &board, player);
        }
        player = !player;
        if !rules::has_legal(&board, player) {
            player = !player;
        }
    }
    let (white, black) = count_winnings(&board);
    board_class::print_board(&board, player);
    if white == black {println!("Tie-------------------\nWhite: {}\nBlack: {}", white, black)}
    if white > black {println!("White wins-------------------\nWhite: {}\nBlack: {}", white, black)}
    if white < black {println!("Black wins-------------------\nWhite: {}\nBlack: {}", white, black)}
}

#[cfg(test)]
mod test{

}
/*
test graveyard:

    println!("\nFor black pieces: ");
    for row in 0..8{
        for col in 0..8{ 
            if rules::is_legal(row, col, &board, false) { print!("k "); }
            else {print!("x ");}
        }
        println!();
    }
    println!("\nFor white pieces: ");
    for row in 0..8{
        for col in 0..8{ 
            if rules::is_legal(row, col, &board, true) { print!("k "); }
            else {print!("x ");}
        }
        println!();
    }
    board[2][3] = Some(true);
    board_class::print_board(&board);
    println!("\nFor black pieces: ");
    for row in 0..8{
        for col in 0..8{ 
            if rules::is_legal(row, col, &board, false) { print!("k "); }
            else {print!("x ");}
        }
        println!();
    }
    println!("\nFor white pieces: ");
    for row in 0..8{
        for col in 0..8{ 
            if rules::is_legal(row, col, &board, true) { print!("k "); }
            else {print!("x ");}
        }
        println!();
    }

    let (mut col, mut row) = input_move::get_move(&board, false);
    board = input_move::place_chip(row, col, &board, false);
    board_class::print_board(&board);
    (col, row) = input_move::get_move(&board, true);
    board = input_move::place_chip(row, col, &board, true);
    board_class::print_board(&board);
    (col, row) = input_move::get_move(&board, false);
    board = input_move::place_chip(row, col, &board, false);
    board_class::print_board(&board);
    (col, row) = input_move::get_move(&board, true);
    board = input_move::place_chip(row, col, &board, true);
    board_class::print_board(&board);
    */