// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let char_vec: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    if char_vec.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(char_vec.len()));
    }
    let mut flat_nums = String::new();

    for line in 0..char_vec.len() / 4 {
        if char_vec[line].len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(char_vec[0].len()));
        }
        for line_len in 0..char_vec[0].len() / 3 {
            let mut single_num = String::new();
            for col in line_len * 3..line_len * 3 + 3 {
                for row in line * 4..line * 4 + 4 {
                    single_num.push(char_vec[row][col]);
                }
            }
            flat_nums.push(match single_num.as_ref() {
                " || _ _  || " => '0',
                "         || " => '1',
                "  | ___  |  " => '2',
                "    ___  || " => '3',
                " |   _   || " => '4',
                " |  ___   | " => '5',
                " || ___   | " => '6',
                "    _    || " => '7',
                " || ___  || " => '8',
                " |  ___  || " => '9',
                _ => '?',
            });
        }
        flat_nums.push(',');
    }
    flat_nums.pop();

    Ok(flat_nums)
}
