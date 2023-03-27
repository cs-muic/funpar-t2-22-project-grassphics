use std::collections::{BTreeMap, HashSet};

use crate::one_dim;

//Checking: sta+mob -> sta+cor -> chip
//will be done sequentially to reduce overhead
pub fn score_count(board: &Vec<Option<bool>>, player: bool, turn_count: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> i32 {
    if turn_count < 15 {
        (early_game(board, player, map_val) * 100_000_000.0) as i32
    }
    else if turn_count < 55 {
        (mid_game(board, player, map_val) * 100_000_000.0) as i32
    }
    else {
        (end_game(board, player) * 100_000_000.0) as i32
    }
}

//mob + map + corners
fn early_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut t_mob = 0.0;
    let mut mob = 0.0;
    let mut t_map = 0.0;
    let mut map = 0.0;
    let mut cor = 0.0;
    let mut t_cor = 0.0;
    let mut pcor = 0.0;
    let mut t_pcor = 0.0;

    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => {
                    let (x, y) = (spot%8, spot/8);
                    if one_dim::is_legal_flat(y, x, board, player){
                        t_mob += 1.0;
                        mob += 1.0;
                    }
                    else if one_dim::is_legal_flat(y, x, board, !player){
                        t_mob += 1.0;
                        mob -= 1.0;
                    }
                    let key = *k as f64;
                    if key == 4.0{
                        match spot{
                            0 => {
                                for pot in vec![1, 8, 9]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = if pot == 9 { 4.0 } else { 1.0 };
                                        if v != player { pcor += diag } else { pcor -= diag }
                                        t_pcor += diag
                                    }
                                   } 
                                }
                            },
                            7 => {
                                for pot in vec![6, 14, 15]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = if pot == 14 { 4.0 } else { 1.0 };
                                        if v != player { pcor += diag } else { pcor -= diag }
                                        t_pcor += diag
                                    }
                                   } 
                                }
                            },
                            56 => for pot in vec![57, 48, 49]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = if pot == 49 { 10.0 } else { 1.0 };
                                    if v != player { pcor += diag } else { pcor -= diag }
                                    t_pcor += diag
                                 }
                                } 
                             },
                            63 => for pot in vec![62, 54, 55]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = if pot == 54 { 10.0 } else { 1.0 };
                                     if v != player { pcor += diag } else { pcor -= diag }
                                     t_pcor += diag
                                 }
                                } 
                             },
                            _ => (),
                        }
                    }
                },
                Some (v) => {
                    let key = *k as f64;
                    if key == 4.0{
                        if v == player { cor += 1.0 } else { cor -= 1.0 }
                        t_cor += 1.0
                    }

                    if key < 0.0{
                        if v != player { map += key}
                        t_map -= key
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
    let cor_mod = if t_cor != 0.0 {cor/t_cor} else {0.0};
    let pcor_mod = if t_pcor != 0.0 {pcor/t_pcor} else {0.0};
    mob_mod *0.075 + map_mod*0.025 + cor_mod*0.6 + pcor_mod*0.3
}

//map + cor 
fn mid_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut cor = 0.0;
    let mut t_cor = 0.0;
    let mut pcor = 0.0;
    let mut t_pcor = 0.0;

    let mut map = 0.0;
    let mut t_map = 0.0;

    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => {
                    let key = *k as f64;
                    if key == 4.0{
                        match spot{
                            0 => {
                                for pot in vec![1, 8, 9]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = if pot == 9 { 4.0 } else { 1.0 };
                                        if v != player { pcor += diag } else { pcor -= diag }
                                        t_pcor += diag
                                    }
                                   } 
                                }
                            },
                            7 => {
                                for pot in vec![6, 14, 15]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = if pot == 14 { 4.0 } else { 1.0 };
                                        if v != player { pcor += diag } else { pcor -= diag }
                                        t_pcor += diag
                                    }
                                   } 
                                }
                            },
                            56 => for pot in vec![57, 48, 49]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = if pot == 49 { 4.0 } else { 1.0 };
                                    if v != player { pcor += diag } else { pcor -= diag }
                                    t_pcor += diag
                                 }
                                } 
                             },
                            63 => for pot in vec![62, 54, 55]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = if pot == 54 { 4.0 } else { 1.0 };
                                     if v != player { pcor += diag } else { pcor -= diag }
                                     t_pcor += diag
                                 }
                                } 
                             },
                            _ => (),
                        }
                    }
                },
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
    let pcor_mod = if t_pcor != 0.0 {pcor/t_pcor} else {0.0};
    cor_mod * 0.8 + map_mod * 0.025 + pcor_mod * 0.175
}

//chip + cor
fn end_game(board: &Vec<Option<bool>>, player: bool) -> f64 {
    let corners = vec![0, 7, 56, 63];
    let mut t_chip = 0.0;
    let mut chip = 0.0;
    let mut t_cor = 0.0;
    let mut cor = 0.0;
    (0..64).for_each(|f| {
        match board[f]{
            None => (),
            Some(v) => {
                if v == player { chip += 1.0 } else { chip -= 1.0 }
                t_chip += 1.0;
                
                if corners.contains(&f) {
                    if v == player { cor += 1.0 } else { cor -= 1.0 }
                    t_cor += 1.0;
                }
            }
        }
    });
    let mod_chip = if t_chip != 0.0 { chip / t_chip } else { 0.0 };
    let mod_cor = if t_cor != 0.0 { cor / t_cor } else { 0.0 };
    mod_chip * 0.25 + mod_cor * 0.75
}
