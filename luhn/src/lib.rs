/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.matches(char::is_numeric).count() > 1 && code
        .chars()
        .filter(|s| !s.is_whitespace())
        .rev()
        .map(|y| y.to_digit(10))
        .enumerate()
        .try_fold(0, |acc, (i, x)| {
            x.map(|a| match i % 2 {
                0 => acc + a,
                _ if a * 2 > 9 => acc + a * 2 - 9,
                _ => acc + a * 2,
            })
        }).map(|x| x % 10 == 0)
        .unwrap_or(false)
}
