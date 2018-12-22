use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code.matches(char::is_numeric).count() > 1 && self
            .code
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let v = input.to_string();
        Luhn { code: v }
    }
}
