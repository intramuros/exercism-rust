use std::char;
use std::iter;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|s| match s {
            '0'...'9' => Some(s),
            'a'...'z' | 'A'...'Z' => s
                .to_digit(36)
                .and_then(|num| char::from_digit(35 - num + 10, 36)),
            _ => None,
        }).enumerate()
        .flat_map(|(i, x)| {
            iter::once(' ')
                .filter(move |_y| i % 5 == 0 && i != 0)
                .chain(iter::once(x))
        }).collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|s| match s {
            '0'...'9' => Some(s),
            'a'...'z' | 'A'...'Z' => s
                .to_digit(36)
                .and_then(|num| char::from_digit(35 - num + 10, 36)),
            _ => None,
        }).collect::<String>()
}
