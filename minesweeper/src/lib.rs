pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let bool_array = minefield
        .iter()
        .map(|line| {
            line.chars()
                .map(|entry| entry == '*')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<_>>();
    bool_array
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, value)| {
                    if !value {
                        match get_neighbour_indices(
                            row_index,
                            col_index,
                            minefield.len(),
                            minefield[0].len(),
                        )
                        .filter(|&(r_ind, c_ind)| bool_array[r_ind][c_ind])
                        .count()
                        {
                            0 => return ' ',
                            num_mines @ 1..=8 => {
                                return std::char::from_digit(num_mines as u32, 10).unwrap()
                            }
                            _ => unreachable!(),
                        };
                    }
                    return '*';
                })
                .collect::<String>()
        })
        .collect()
}

fn get_neighbour_indices(
    row_index: usize,
    col_index: usize,
    field_height: usize,
    field_width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (row_index as i8 - 1..row_index as i8 + 2).flat_map(move |r_ind| {
        (col_index as i8 - 1..col_index as i8 + 2).filter_map(move |c_ind| {
            if r_ind >= 0
                && c_ind >= 0
                && r_ind < field_height as i8
                && c_ind < field_width as i8
                && !(r_ind == row_index as i8 && c_ind == col_index as i8)
            {
                Some((r_ind as usize, c_ind as usize))
            } else {
                None
            }
        })
    })
}
