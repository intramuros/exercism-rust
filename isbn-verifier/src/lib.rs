/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars().filter(|x| x.is_numeric() || *x == 'X').count() == 10
        && isbn
            .chars()
            .filter(|c| c.is_numeric() || *c == 'X')
            .enumerate()
            .filter_map(|(i, x)| match x {
                'X' if i == 9 => Some(10),
                '0'...'9' => x.to_digit(10),
                _ => None,
            }).zip((1..=10).rev())
            .map(|(a, b)| a * b)
            .sum::<u32>()
            % 11
            == 0
}
