use std::collections::{BTreeMap, HashSet};
use crate::{one_dim, ab_score};
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
 * 
 * ALPHA BETA ADDITIONS: 
 * implementation of the alpha-beta idea
 */
#[allow(dead_code)]
pub fn minimax(board: &Vec<Vec<Option<bool>>>, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> (usize, usize){ 
    let f_board: Vec<Option<bool>> = one_dim::flatten_board(board);
    let routes = one_dim::all_legal_flat(&f_board, BOT_SIDE);    
    if routes.len() == 1 {return to_pos(*routes.get(0).unwrap())} //TODO: Handle proper return, OR HANDLE THIS IN THE MAIN FUNCTION :3 maybe
    println!("routes: {:?}",routes); // for debugging

    let alpha = -1_000_000_000;
    let beta = 1_000_000_000;
    // call minimax help here in parallel with enumerate to find the best possible move to proceed
    let (_scores, x, y) = ab_handler(f_board,alpha,beta,BOT_SIDE,1,moves,&map_val,false).unwrap();

    println!("{}, {}", x, y);
    (x, y)
}

//Since alpha-beta is faster, but considerably dumber, we'll try going deeper to balance it out
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
fn minimax_help(board: Vec<Option<bool>>, alpha: i32, beta: i32, turn: bool, x: usize, y: usize, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> Option<(i32, usize, usize)>{ 
    //If depth exceeds the amount of depth we're going form return the result scores
    if depth >= DEPTH_LEVEL { return 
        Some((ab_score::score_count(&board, BOT_SIDE, moves, &map_val), x, y));
    }
    let mut scores: Option<(i32, usize, usize)> = None; //tmp value assignment

    //check how many vectors the opponent can go through
    let routes = one_dim::all_legal_flat(&board, !turn);

    //skip this "calculation" since it is fixed
    if routes.len() == 1 {
        let (thisx,thisy)  = to_pos(*routes.get(0).unwrap());
        let n_board = one_dim::place_chip_flat(thisx, thisy, &board , !turn);
        return match ab_handler(n_board, alpha, beta, turn, depth, moves, map_val, true){
            None => None,
            Some((v, _x, _y)) => Some((v, x, y))
        }
    }
    else if routes.len() == 0{
        //return the points accordingly eg if win 999 lose -999 tie then some arbitary number or find a better way to handle this
        if let Some(i) = game_result(&board, turn) { return Some((i, x, y)) ;}
        else {
            return minimax_help(board, alpha, beta, !turn, x, y, depth, moves+1, &map_val);
        } //Skip move
    }
    else if depth <= DEPTH_LEVEL-2 {
        scores = ab_handler(board, alpha, beta, !turn, depth+1, moves+1, &map_val, false);
    }
    else if depth == DEPTH_LEVEL-1 {
        if routes.len() >= 16 { 
            scores = ab_handler(board, alpha, beta, !turn, depth+1, moves+1, &map_val, false);
        } // chunk this and call several seq instead
        else {scores = ab_handler(board, alpha, beta, !turn, depth+1, moves+1, &map_val, true);} 
    }
    else { return Some((ab_score::score_count(&board, BOT_SIDE, moves, &map_val), x, y));} //final score calculation at max depth
    //I personally don't think this will be called but hey, a safety net won't hurt
    
    match scores {
        None => None,
        Some((sc, _x, _y)) => Some((sc, x, y))
    }
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

fn seq_search(board: &Vec<Option<bool>>, alpha: i32, beta: i32, routes: &HashSet<usize>, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> Vec<(i32, usize, usize)>{
    routes
    .iter()
    .map(|position | {
        let (x,y) = to_pos(*position);
        if !one_dim::is_legal_flat(y, x, &board, turn) { return None }
        
        let n_board = one_dim::place_chip_flat(x, y, &board, turn);
        minimax_help(n_board, alpha, beta, turn, x, y, depth, moves, &map_val)
        }
    ).filter_map(|k| k)
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
 * 
 * for ab: also returns the position, alpha, beta
 */

fn par_search(board: &Vec<Option<bool>>, alpha: i32, beta: i32, routes: &HashSet<usize>, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>)-> Vec<(i32, usize, usize)>{
    routes
    .par_iter()
    .map(|position | {
        let (x,y) = to_pos(*position);
        if !one_dim::is_legal_flat(y, x, &board, turn) { return None }

        let n_board = one_dim::place_chip_flat(x, y, &board, turn);
        minimax_help(n_board, alpha, beta, turn, x, y, depth, moves, &map_val)
        }
    ).filter_map(|k| k)
    .collect()
}

/// For AB, proceed the call in sequential chunks, returns the x, y, as well as the score
fn ab_handler(board: Vec<Option<bool>>, alpha: i32, beta: i32, turn: bool, depth: u8, moves: usize, map_val: &BTreeMap<i32, HashSet<usize>>, seq: bool)-> Option<(i32, usize, usize)>{
    let (mut alpha, mut beta) = (alpha, beta);
    let mut answer = None;
    let mut pruned = false;
    map_val.iter().rev().for_each(|(k, v)| {
        let mut vn = v.clone();
        if !pruned{
            if *k == 4 {
                for spot in v{
                    match board[*spot]{
                        None => (),
                        Some(val) => if val == turn {
                            match spot{
                                0 => {
                                    vn.insert(1);
                                    vn.insert(8);
                                    vn.insert(9);
                                }
                                7 => {
                                    vn.insert(6);
                                    vn.insert(14);
                                    vn.insert(15);
                                }
                                56 => {
                                    vn.insert(48);
                                    vn.insert(49);
                                    vn.insert(57);
                                }
                                63 => {
                                    vn.insert(55);
                                    vn.insert(56);
                                    vn.insert(62);
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            let scores = if seq { seq_search(&board, alpha, beta, &vn, turn, depth, moves, map_val) } 
            else { par_search(&board, alpha, beta, &vn, turn, depth, moves, map_val) }; 

            if depth == 1 {println!("{:?}", scores);}
            if scores.len() > 0 {
                let mut wanted = if turn == BOT_SIDE { *scores.iter().max().unwrap() } 
                else { *scores.iter().min().unwrap() };

                if vec![9, 14, 49, 54].contains(&(wanted.1+wanted.2*8)) { 
                    wanted.0 = if turn == BOT_SIDE { -999_999_990 }  else { 999_999_990 }
                }
                //bot side is maximizing, else it's minimizing
                if turn == BOT_SIDE {
                    if wanted.0 >= beta { 
                        pruned = true;
                    }
                    else if wanted.0 > alpha {
                        alpha = wanted.0;
                        answer = Some(wanted);
                    }
                }
                else {
                    if wanted.0 <= alpha { 
                        pruned = true;
                    }
                    else if wanted.0 < beta {
                        beta = wanted.0;
                        answer = Some(wanted);
                    }
                }
            }
        }
    });
    answer
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
