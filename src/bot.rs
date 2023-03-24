use crate::{rules, board_class};
use crate::rayon::Iter; //and other things

/** MINIMAX CODE NOTES
 * Objective: 
 *      > Design in such a way that it handles AB in parallel easier
 * Questions: 
 *      > DFS/BFS-esque design might be better in multithreading 
 *      > Any possible ways to handle generating new board from previous
 *        (multiple reads at the same time?) such that there is less operation 
 *        need to be done.
 * 
 * Current Design: 
 *      > The maximum depth of pre calculated move is 5
 *      > At depth 1-3, the moves will be separated in parallel
 *        if there is only 1 possibility, the move will be done in sequential
 *      > At depth 4-5, the moves will be calculated in sequential unless if 
 *        there are >= 16 moves where each thread calculated 8 moves each. 
 * --------------------------------------------------------------------------------
 */

/**  
 * Basic minimax to find the best possible moves
 * Call this function only when there are > 1 possible moves
 * 
 * board: current board
 * moves: current move count
 * 
 * ↪ return the move to make in (usize,usize)
 */
#[allow(dead_code)]
pub fn minimax(board: &Vec<Vec<Option<bool>>>, moves: u8) -> (usize, usize){
    let f_board: Vec<Option<bool>> = rules::flatten_board(board);
    let routes = rules::all_legal_flat(f_board);    
    if routes.len == 0 {return (routes.get(1))}
    let x = a;
    
}

/**
 * This function should handle spawning threads as well as calling functions 
 * to find the optimal moves.
 * 
 * board: board in the previous step
 * turn: true if max, false if min
 * depth: count current number of turns simulated in depth
 * moves: current move count (use to determine the stage of the game)
 */
#[allow(dead_code)]
fn minimax_help(board: Vec<Option<bool>>, turn: bool, depth: u8, moves: u8){

}

/**
 * Handle moves for max, generate a new Vec of movesets
 * possibly return the new Vec to minimax_help for the next moves.
 */
#[allow(dead_code)]
fn black(board: Vec<String>, p_moveset:moveset){
    
}

/**
 * Handle moves for min, generate a new Vec of movesets
 * possibly return the new Vec to minimax_help for the next moves.
 */
#[allow(dead_code)]
fn white(board: Vec<String>, p_moveset:moveset){

}
