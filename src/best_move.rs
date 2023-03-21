use crate::{rules, board_class};
use crate::rayon::Iter; //and other things

// MINMAX Code Skeleton
/**
 * Objective: Design in such a way that it handles AB in parallel easier
 * Questions: 
 *      > DFS/BFS-esque design might be better in multithreading 
 *      > Any possible ways to handle generating new board from previous
 *        (multiple reads at the same time?) such that there is less operation need to
 *        be done.
 */

#[allow(dead_code)]
/**
 * This class stores the moves, value of the total set, and steps required to win
 */
struct moveset { 
    moves: Vec<String>,
    value: u32,
    steps: u32
}

/**
 * Basic minmax to find the best possible moves 
 */
#[allow(dead_code)]
pub fn minmax(board: Vec<String>){

}

/**
 * This function should handle spawning threads as well as calling functions 
 * to find the optimal moves.
 * 
 * board: board in the previous step
 * p_moveset: previous moveset
 * turn: true if max, false if min
 */
#[allow(dead_code)]
fn minmax_help(board: Vec<String>, p_moveset: moveset, turn: bool){

}

/**
 * Handle moves for max, generate a new Vec of movesets
 * possibly return the new Vec to minmax_help for the next moves.
 */
#[allow(dead_code)]
fn black(board: Vec<String>, p_moveset:moveset){
    
}

/**
 * Handle moves for min, generate a new Vec of movesets
 * possibly return the new Vec to minmax_help for the next moves.
 */
#[allow(dead_code)]
fn white(board: Vec<String>, p_moveset:moveset){

}

/**
 * use to generate new boards in order to make testing for the 
 * next moves easier
 */
#[allow(dead_code)]
fn gen_pusedo_board() -> Vec<Vec<Option<bool>>>{
    
}
