use std::collections::HashMap;

#[derive(Debug)]
struct PairEntry {
    squares: usize,
    prev_row: usize,
}


pub fn count(lines: &[&str]) -> u32 {
    let mut num_squares = 0;
    let mat = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut pair_set: HashMap<[usize; 2], PairEntry> = HashMap::new();
    for row in 0..mat.len() {
        let mut cur_corners = vec![];
        for col in 0..mat[0].len() {
            if mat[row][col] == '+' {
                cur_corners.push(col);
            }
        }
        for pair in
            cur_corners.iter().enumerate().flat_map(|(i, elem)| {
                cur_corners.iter().enumerate().filter_map(move |(j, el)| {
                    if j > i {
                        Some([*elem, *el])
                    } else {
                        None
                    }
                })
            })
        {
            if let Some(p) =  pair_set.get_mut(&pair){
                // check if all the chars are valid in between rows
                let mut true_square = true;
                for &sq_col in &pair{
                    for sq_row in p.prev_row + 1..row {
                        if mat[sq_row][sq_col] != '|' && mat[sq_row][sq_col] != '+'{
                            true_square = false;
                            break;
                        }
                    }
                }
                if true_square {
                   p.squares += 1;
                   num_squares += p.squares;
                   p.prev_row = row;
                }
            } else {
                pair_set.insert(pair, PairEntry{squares: 0, prev_row: row, });
            }
        }
    }
    num_squares as u32
}
