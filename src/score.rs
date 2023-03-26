use crate::one_dim;

pub fn score_count(board: &Vec<Option<bool>>, player: bool, turn_count: usize) -> i32 {
    let mut score = 0_i32;
    //mobility count: Todo: Corner and corne-ighbour advantages and disadvantages
    if turn_count <= 40 {
        score += one_dim::all_legal_flat(&board, player).len() as i32;
        score -= one_dim::all_legal_flat(&board, !player).len() as i32;
    }
    //chip count: Todo: priority and non-priority
    if turn_count >= 20 {
        score += board.into_iter().fold(0, |base, spot| match spot{
            None => base,
            Some(v) => if *v==player { base + 1 } else { base - 1 }
        });
    }
    score

    // Haven't came up with a good enough matrix so its gonna stay like this for now... Too Bad! ^^
    // TODO: Make this one more fine tuned
}