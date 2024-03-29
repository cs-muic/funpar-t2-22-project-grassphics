use std::collections::{BTreeMap, HashSet};
use crate::{one_dim,score};
use rayon::iter::*;

/** MINIMAX CODE NOTES
 * Direction: 
 *      > Design in such a way that it handles AB in parallel easier?
 * Questions: 
 *      > DFS/BFS-esque design might be better in multithreading 
 *      > Any possible ways to handle generating new board from previous
 *        (multiple reads at the same time?) such that there is less operation 
 *        need to be done.
 *      > Should intermediate steps be calculated accumulatively? 
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
 *      > game_result(board: &Vec<Option<bool>>) -> Option(u8) // not_ended None, won Some(1), lose Some(2), tie Some(0) 
 * ---------------------------------------------------------------------------------------------------------------------
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
pub fn minimax(board: &Vec<Vec<Option<bool>>>, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> (usize, usize){ 
    let f_board: Vec<Option<bool>> = one_dim::flatten_board(board);
    let routes = one_dim::all_legal_flat(&f_board, BOT_SIDE);    
    if routes.len() == 1 {return to_pos(*routes.get(0).unwrap())} //TODO: Handle proper return, OR HANDLE THIS IN THE MAIN FUNCTION :3 maybe
    println!("routes: {:?}",routes); // for debugging
     
    // call minimax help here in parallel with enumerate to find the best possible move to proceed
    let scores = par_search(f_board,&routes,BOT_SIDE,1,moves,&map_val);
    println!("scores: {:?}", scores);
    let best_move: usize = (1..scores.len()).into_iter().fold(
        (0, scores[0]), |base, i| {
            if (scores[i] > scores[0]) == BOT_SIDE { (i, scores[i]) }
            else { base }
        }
    ).0;

    to_pos(*routes.get(best_move).unwrap())
}

const DEPTH_LEVEL: u8 = 5;
const BOT_SIDE: bool = true;
/**
 * This function handle whether the branching should be parallelized
 * or serialized to find the optimal moves.
 * 
 * board: board in the previous step
 * turn: true if max, false if min <going by the assumption the function will always starts with true due to it being called from the bot>
 * depth: count current number of turns simulated in depth
 * moves: current move count (use to determine the stage of the game)
 * 
 * ↪ return the score of the current path as i32 
 */
#[allow(dead_code)]
#[allow(unused_assignments)]
fn minimax_help(board: Vec<Option<bool>>, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> i32{ 
    //If depth exceeds the amount of depth we're going form return the result scores
    if depth >= DEPTH_LEVEL {return score::score_count(&board, BOT_SIDE, moves, &map_val);}
    let mut scores: Vec<i32> = Vec::new(); //tmp value assignment

    //check how many vectors the opponent can go through
    let routes = one_dim::all_legal_flat(&board, !turn);

    //skip this "calculation" since it is fixed
    if routes.len() == 1 {
        let (x,y)  = to_pos(*routes.get(0).unwrap());
        let n_board = one_dim::place_chip_flat(x, y, &board , !turn);
        return minimax_help(n_board, !turn, depth, moves+1, &map_val);
    }
    else if routes.len() == 0{
        //return the points accordingly eg if win 999 lose -999 tie then some arbitary number or find a better way to handle this
        if let Some(i) = game_result(&board, turn) { return i ;}
        else {
            return minimax_help(board, !turn, depth, moves+1, &map_val);
        } //Skip move
    }
    else if depth <= DEPTH_LEVEL-2 {
        scores = par_search(board, &routes, !turn, depth+1, moves+1, &map_val)
    }
    else if depth == DEPTH_LEVEL-1 {
        if routes.len() >= 16 { 
            scores = par_search(board, &routes, !turn, depth+1, moves+1, &map_val)
        } // chunk this and call several seq instead
        else {scores = seq_search(board, &routes, !turn, depth+1, moves+1, &map_val)} 
    } 
    else if depth == DEPTH_LEVEL { scores = seq_search(board, &routes,!turn, depth+1, moves+1, &map_val)} 
    
    else { return score::score_count(&board, BOT_SIDE, moves, &map_val);} //final score calculation at max depth
    //I personally don't think this will be called but hey, a safety net won't hurt
    
    // return min / max based on whose turn it came from
    if turn == BOT_SIDE { *scores.iter().min().unwrap() }
    else { *scores.iter().max().unwrap() }
}

/**
 * This function calls minimax_help in sequential
 * This funciton will be used at depth 4-5
 * 
 * board: current state of the board
 * turn: min/ max player
 * depth: depth of the simulation
 * moves: current moves in game
 * ↪ returns a Vector of the score
 */

fn seq_search(board: Vec<Option<bool>>, routes: &Vec<usize>, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> Vec<i32>{
    routes
    .iter()
    .map(|position | {
        let (x,y) = to_pos(*position);
        let n_board = one_dim::place_chip_flat(x, y, &board, turn);
        minimax_help(n_board, turn, depth, moves, &map_val)
        }
    )
    .collect()
}

/**
 * This function calls minimax_help in parallel
 * This function will be used from depth 1-3 and 4 on special case where there are >= 16 possible moves
 * 
 * board: current state of the board
 * turn: min/ max player
 * depth: depth of the simulation
 * moves: current moves in game
 * ↪ returns a Vector of the score
 */

fn par_search(board: Vec<Option<bool>>, routes: &Vec<usize>, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>)-> Vec<i32>{
    routes
    .par_iter()
    .map(|position | {
        let (x,y) = to_pos(*position);
        let n_board = one_dim::place_chip_flat(x, y, &board, turn);
        minimax_help(n_board, turn, depth, moves, &map_val)
        }
    )
    .collect()
}

/**
 * checking game result in case of an end
 * 
 * will return none in case the game is not over
 */
#[allow(dead_code)]
#[allow(unused_variables)]
fn game_result(board :&Vec<Option<bool>>, turn: bool) -> Option<i32>{
    if one_dim::all_legal_flat(board, turn).len() != 0 { return None }
    let (white, black) = one_dim::count_winnings_flat(&board);
    if (white > black) == BOT_SIDE { Some(999_999_999) }
    else if (white < black) == BOT_SIDE { Some(-999_999_999) }
    else { Some(0) }
}

/**
 * This function returns the 2D position based from 1D position
 * 
 * flat_pos: accepts usize 1D position
 * ↪ 2D position in tuple of usize
 */
fn to_pos(flat_pos: usize) -> (usize,usize){ 
    (flat_pos % 8, flat_pos/8) 
}
