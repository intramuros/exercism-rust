pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Check if matrix is empty
    if input[0].is_empty() {
        return Vec::new();
    }

    // Iterate through each row and pick up indices of maximum values
    let mut y: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            let cur_max = row.iter().max().unwrap();
            row.iter()
                .enumerate()
                .filter_map(move |(j, elem)| if elem == cur_max { Some((i, j)) } else { None })
        })
        .collect();

    // Remove indices of non-saddle values from the list of row maxima
    y.retain(|&(i, j)| {
        for row in 0..input.len() {
            if input[row][j] < input[i][j] {
                return false;
            }
        }
        true
    });
    y
}
