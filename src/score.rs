use std::collections::{BTreeMap, HashSet};

//Checking: sta+mob -> sta+cor -> chip
//will be done sequentially to reduce overhead
pub fn score_count(board: &Vec<Option<bool>>, player: bool, turn_count: usize, map_val: &BTreeMap<i32, HashSet<usize>>) -> i32 {
    if turn_count < 20 {
        (early_game(board, player, map_val) * 10_000_000.0) as i32
    }
    else if turn_count < 55 {
        (mid_game(board, player, map_val) * 10_000_000.0) as i32
    }
    else {
        (end_game(board, player) * 10_000_000.0) as i32
    }
}

//mob + sta + cor
fn early_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut t_mob = 0.0;
    let mut mob = 0.0;
    let mut cor = 0.0;
    let mut t_cor = 0.0;
    let mut pcor = 0.0;
    let mut t_pcor = 0.0;
    let mut p_unst: HashSet<usize> = HashSet::new();
    let mut e_unst: HashSet<usize> = HashSet::new();
    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => {
                    if unstable_check(*spot, board, player, &mut e_unst){
                        t_mob += 1.0;
                        mob += 1.0;
                    }
                    if unstable_check(*spot, board, !player, &mut p_unst){
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
                                        if v != player { pcor += 1.0 } else { pcor -= 1.0 }
                                        t_pcor += 1.0
                                    }
                                   } 
                                }
                            },
                            7 => {
                                for pot in vec![6, 14, 15]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = 1.0;
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
                                    let diag = 1.0;
                                    if v != player { pcor += diag } else { pcor -= diag }
                                    t_pcor += diag
                                 }
                                } 
                             },
                            63 => for pot in vec![62, 54, 55]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = 1.0;
                                     if v != player { pcor += diag } else { pcor -= diag }
                                     t_pcor += diag
                                 }
                                } 
                             },
                            _ => (),
                        }
                    }
                },
                Some(v) => {
                    let key = *k as f64;
                    if key == 4.0{
                        if v == player { cor += 1.0 } else { cor -= 1.0 }
                        t_cor += 1.0
                    }
                },
            }
        }
    }

    let (sta_p, sta_e) = stable_count(board, player);
    let mob_mod = if t_mob != 0.0 {mob/t_mob} else {0.0};
    let usta_mod = if p_unst.len() + e_unst.len() == 0 {
        0.0
    }
    else {
        (e_unst.len() as f64 - p_unst.len() as f64) /
        ((p_unst.len() + e_unst.len()) as f64)
    };
    let sta_mod = if sta_p + sta_e == 0.0 { 0.0 } else {(sta_p - sta_e) / (sta_p + sta_e)};
    let cor_mod = if t_cor != 0.0 {cor/t_cor} else {0.0};
    let pcor_mod = if t_pcor != 0.0 {pcor/t_pcor} else {0.0};
    if sta_mod != 0.0 { cor_mod * 0.25 + sta_mod * 0.75 }
    else { pcor_mod * 0.015 + usta_mod * 0.01 + mob_mod * 0.005 }
}

//sta + cor 
fn mid_game(board: &Vec<Option<bool>>, player: bool, map_val: &BTreeMap<i32, HashSet<usize>>) -> f64 {
    let mut cor = 0.0;
    let mut t_cor = 0.0;
    let mut pcor = 0.0;
    let mut t_pcor = 0.0;

    let mut p_unst: HashSet<usize> = HashSet::new();
    let mut e_unst: HashSet<usize> = HashSet::new();

    for (k, v) in map_val{
        for spot in v{
            match board[*spot]{
                None => {
                    unstable_check(*spot, board, player, &mut e_unst);
                    unstable_check(*spot, board, !player, &mut p_unst);

                    let key = *k as f64;
                    if key == 4.0{
                        match spot{
                            0 => {
                                for pot in vec![1, 8, 9]{
                                   match board[pot]{
                                    None => (),
                                    Some(v) => {
                                        let diag = 1.0;
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
                                        let diag = 1.0;
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
                                    let diag = 1.0;
                                    if v != player { pcor += diag } else { pcor -= diag }
                                    t_pcor += diag
                                 }
                                } 
                             },
                            63 => for pot in vec![62, 54, 55]{
                                match board[pot]{
                                 None => (),
                                 Some(v) => {
                                    let diag = 1.0;
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
                }
            }
        }
    }

    let (sta_p, sta_e) = stable_count(board, player);
    let cor_mod = if t_cor != 0.0 {cor/t_cor} else {0.0};
    let pcor_mod = if t_pcor != 0.0 {pcor/t_pcor} else {0.0};
    let usta_mod = if p_unst.len() + e_unst.len() == 0 {
        0.0
    }
    else {
        (e_unst.len() as f64 - p_unst.len() as f64) /
        ((p_unst.len() + e_unst.len()) as f64)
    };
    let sta_mod = if sta_p + sta_e == 0.0 { 0.0 } else {(sta_p - sta_e) / (sta_p + sta_e)};
    if sta_mod != 0.0 { cor_mod * 0.25 + sta_mod * 0.75 }
    else { pcor_mod * 0.015 + usta_mod * 0.015 }
}

//chip
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
    mod_chip * 0.75 + mod_cor * 0.25
}

//returns the stable count for the enemy and the player
fn stable_count(board: &Vec<Option<bool>>, player: bool) -> (f64, f64){
    let mut ps: HashSet<usize> = HashSet::new();
    let mut es: HashSet<usize> = HashSet::new();

    for spot in vec![0, 7, 56, 63] {
        match board[spot]{
            None => (),
            Some(v) => {
                if v == player {
                    match spot{
                        0 => {
                            stability(board, v, 0, 8, 1, &mut ps);
                        }
                        7 => {stability(board, v, 7, -1, 8, &mut ps);
                        }
                        56 => {
                            stability(board, v, 56, -8, 1, &mut ps);
                        }
                        63 => {
                            stability(board, v, 63, -8, -1, &mut ps);
                        }
                        _ => (),
                    }
                }
                else {
                    match spot{
                        0 => {
                            stability(board, v, 0, 8, 1, &mut es);
                        }
                        7 => {stability(board, v, 7, -1, 8, &mut es);
                        }
                        56 => {
                            stability(board, v, 56, -8, 1, &mut es);
                        }
                        63 => {
                            stability(board, v, 63, -8, -1, &mut es);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    (ps.len() as f64, es.len() as f64)
}

//updates the hashset according to which pieces are stable
fn stability(board: &Vec<Option<bool>>, color: bool, corner: usize, prop: isize, check: isize,
pieces: &mut HashSet<usize>){
    let mut height = 8;
    for i in 0..8{
        for j in 0..height{
            if height > j {
                match board[(corner as isize + i*prop + j*check) as usize]{
                    None => height = j,
                    Some(v) => {
                        if v == color {pieces.insert((corner as isize + i*prop + j*check) as usize);}
                        else {height = j;}
                    }
                }
            }
        }
    }
}

//check which pieces are unstable according to a player, returns whether move is legal or not
fn unstable_check(position: usize, board: &Vec<Option<bool>>, player: bool, unst: &mut HashSet<usize>) -> bool{
    let mut legal = false;
    match board[position]{
        Some(_v) => (),
        None => {
            for row in 0..3{
                for col in 0..3{
                    if !(row == 1 && col == 1) && flipper_flat(position%8, position/8, &board, player, false, row, col, unst, vec![]) { legal = true }
                }
            }
        },
    }
    legal
}

fn flipper_flat(x: usize, y: usize, board: &Vec<Option<bool>>, player: bool, pinged: bool, row: usize, col: usize, unst: &mut HashSet<usize>, pieces: Vec<usize>) -> bool {
    if (col + y) < 1 || (col + y) > 8 || (row + x) < 1 || (row + x) > 8 { return false }
    match board[(x+row-1)*8 + y+col-1]{
        None => false,
        Some(v) => {
            if pinged && (player != v) { 
                for u in pieces { unst.insert(u); }
                true 
            }
            else if (player == v) && !pinged { false }
            else { flipper_flat(x + row-1, y+col-1, board, v, true, row, col, unst, [pieces, vec![x+y*8]].concat()) }
        }
    }
}
