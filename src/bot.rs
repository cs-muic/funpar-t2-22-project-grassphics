use crate::{rules};
use rayon::iter::*;

/** MINIMAX CODE NOTES
 * Direction: 
 *      > Design in such a way that it handles AB in parallel easier?
 * Questions: 
 *      > DFS/BFS-esque design might be better in multithreading 
 *      > Any possible ways to handle generating new board from previous
 *        (multiple reads at the same time?) such that there is less operation 
 *        need to be done.
 *      > Should intermediate steps be calculated too? 
 * 
 * Current Design: 
 *      > The maximum depth of pre calculated move is 5
 *      > At depth 1-3, the moves will be separated in parallel
 *        if there is only 1 possibility, the move will be done in sequential using the same thread
 *      > At depth 4, the moves will be calculated in sequential unless if 
 *        there are >= 16 moves where each thread calculated 8 moves each. 
 *      > At depth 5, the moves will be calculated sequentially
 * 
 * Missing Functions:
 *      > place_disc_flatten(board: Vec<Option<bool>>, current_move: usize, turn: bool) -> Vec<Option<bool>>
 *      > game_result(board: &Vec<Option<bool>>) -> Option(u8) // not_ended None, won Some(1), lose Some(2), tie Some(0) 
 *      > scoring function <compare both position and current state>
 *      > to_position(position: usize) -> (usize,usize)
 * --------------------------------------------------------------------------------
 */

/**  
 * Basic minimax to find the best possible moves
 * Call this function only when there are > 1 possible moves
 * ※ MAYBE accept numbers of white and black as param is a good idea to reduce computation required
 * 
 * board: current board
 * moves: current move count
 * 
 * ↪ return the move to make in (usize,usize)
 */
#[allow(dead_code)]
pub fn minimax(board: &Vec<Vec<Option<bool>>>, moves: u8) -> (usize, usize){ 
    let f_board: Vec<Option<bool>> = rules::flatten_board(board);
    let routes = rules::all_legal_flat(&f_board, true);    
    if routes.len() == 1 {return to_position(*routes.get(0).unwrap())} //TODO: Handle proper return, OR HANDLE THIS IN THE MAIN FUNCTION

    let best_move = *minimax_help(f_board, true, 1, moves).iter().enumerate().max_by_key(|i,&v| v).unwrap().1;

    to_position(best_move)
}

/**
 * This function handle whether the branching should be parallelized
 * or serialized to find the optimal moves.
 * 
 * board: board in the previous step
 * turn: true if max, false if min
 * depth: count current number of turns simulated in depth
 * moves: current move count (use to determine the stage of the game)
 */
#[allow(dead_code)]
fn minimax_help(board: Vec<Option<bool>>, turn: bool, depth: u8, moves: u8) -> Vec<i32>{
    let routes: Vec<usize> = rules::all_legal_flat(&board, turn);
    //let mut scores: Vec<usize> = Vec::new(); //tmp var

    if routes.len() == 1 { return minimax_help(place_disc_flatten(board, routes.get(0).unwrap(), turn), !turn, depth+1, moves+1);}
    else if routes.len() == 0{
        if let Some(i) = game_result(&board) { Vec::from(i) } //return the points accordingly eg if win 999 lose -999 tie then some arbitary number
        else {return minimax_help(board, !turn, depth+1, moves+1);} //Skip move
    }
    else if depth <= 3 {
        par_search(board, turn, depth, moves)
    }
    else if depth == 4 {
        if routes.len() >= 16 { par_search(board, turn, depth, moves)} 
        else {seq_search(board, turn, depth, moves)} 
    } 
    else if depth == 5 { seq_search(board, turn, depth, moves)}
    //might need to add cases for depth == 6 to handle final calculation (maybe as another function call)
    else {Vec::new()} // this is added just so that rust won't be mad

    /* 
    if turn {*scores.iter().max().unwrap()}
    else {*scores.iter().min().unwrap()}
    */
     
}

/**
 * 
 */
#[allow(dead_code)]
fn seq_search(board: Vec<Option<bool>>, turn: bool, depth: u8, moves: u8) -> Vec<i32>{
    // do we keep all the intermediate maxes and mins to calculate afterward or just calculate together with each move?
    rules::all_legal_flat(&board, turn)
    .iter()
    .map(|position| minimax_help(board, turn, depth, moves))// modification of board is required
    .flatten()
    .collect()
}

/**
 * 
 */
#[allow(dead_code)]
fn par_search(board: Vec<Option<bool>>, turn: bool, depth: u8, moves: u8)-> Vec<i32>{
    // do we keep all the intermediate maxes and mins to calculate afterward or just calculate together with each move?
    rules::all_legal_flat(&board, turn)
    .par_iter()
    .map(|position| minimax_help(board, turn, depth, moves))// modification of board is required
    .flatten()
    .collect()
}

/**
 * 
 */
#[allow(dead_code)]
fn scoring(board: &Vec<Option<bool>>, p_board: &Vec<Option<bool>>) -> i32 {
    0 //TODO: Implement this
}

/**
 * This function returns the 2D position based from 1D position
 */
#[allow(dead_code)]
fn to_position(position: usize) -> (usize,usize){
    (0,0) //TODO: Implement this
}










/// Likely Will be deprecate functions ↓

/**
 * Handle moves for max, generate a new Vec of movesets
 * possibly return the new Vec to minimax_help for the next moves.
 */
#[allow(dead_code)]
fn black(board: Vec<String>){
    
}

/**
 * Handle moves for min, generate a new Vec of movesets
 * possibly return the new Vec to minimax_help for the next moves.
 */
#[allow(dead_code)]
fn white(board: Vec<String>){

}
