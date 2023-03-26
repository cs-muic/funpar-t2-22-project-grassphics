use std::collections::{BTreeMap, HashSet};

use crate::one_dim;

//Checking: sta+mob -> sta+cor -> chip
//will be done sequentially to reduce overhead
pub fn score_count(board: &Vec<Option<bool>>, player: bool, turn_count: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> i32 {
    if turn_count < 20 {
        (early_game(board, player, map_val) * 100_000_000.0) as i32
    }
    else if turn_count < 50 {
        (mid_game(board, player, map_val) * 100_000_000.0) as i32
    }
    else {
        (end_game(board, player) * 100_000_000.0) as i32
    }
}

//mob + map
fn early_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut t_mob = 0.0;
    let mut mob = 0.0;
    let mut t_map = 0.0;
    let mut map = 0.0;

    (0..64).into_iter().for_each(|f| {
        let (x, y) = (f%8, f/8);
        if one_dim::is_legal_flat(x, y, board, player){
            t_mob += 1.0;
            mob += 1.0;
        }
        else if one_dim::is_legal_flat(x, y, board, !player){
            t_mob += 1.0;
            mob -= 1.0;
        }
    });

    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => (),
                Some (v) => {
                    let key = *k as f64;
                    if key < 0.0{
                        if v != player { map += key}
                        t_map += key
                    }
                    else{
                        if v == player { map += key}
                        t_map += key
                    }
                }
            }
        }
    }

    let mob_mod = if t_mob != 0.0 {mob/t_mob} else {0.0};
    let map_mod = if t_map != 0.0 {map/t_map} else {0.0};
    mob_mod *0.75+ map_mod*0.25
}

//map + cor
fn mid_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut cor = 0.0;
    let mut t_cor = 0.0;

    let mut map = 0.0;
    let mut t_map = 0.0;

    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => (),
                Some (v) => {
                    let key = *k as f64;
                    if key == 4.0{
                        if v == player { cor += 1.0 } else { cor -= 1.0 }
                        t_cor += 1.0
                    }
                    if key < 0.0{
                        if v != player { map += key}
                        t_map += key
                    }
                    else{
                        if v == player { map += key}
                        t_map += key
                    }
                }
            }
        }
    }

    let cor_mod = if t_cor != 0.0 {cor/t_cor} else {0.0};
    let map_mod = if t_map != 0.0 {map/t_map} else {0.0};
    cor_mod*0.75+map_mod*0.25
}

//chip
fn end_game(board: &Vec<Option<bool>>, player: bool) -> f64 {
    let mut t_chip = 0.0;
    let mut chip = 0.0;
    board.iter().for_each(|f| {
        match f{
            None => (),
            Some(v) => {
                if *v == player { chip += 1.0 } else { chip -= 1.0 }
                t_chip += 1.0
            }
        }
    });
    if t_chip != 0.0 { chip / t_chip } else { 0.0 }
}