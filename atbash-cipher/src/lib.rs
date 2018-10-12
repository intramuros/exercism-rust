use std::char;
use std::iter;

fn make_iter<'a>(text: &'a str) -> impl Iterator<Item = char> + 'a {
    text.chars().filter_map(|s| match s {
        '0'...'9' => Some(s),
        'a'...'z' | 'A'...'Z' => s
            .to_digit(36)
            .and_then(|num| char::from_digit(35 - num + 10, 36)),
        _ => None,
    })
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    make_iter(plain)
        .enumerate()
        .flat_map(|(i, x)| {
            iter::once(' ')
                .filter(move |_y| i % 5 == 0 && i != 0)
                .chain(iter::once(x))
        }).collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    make_iter(cipher).collect()
}
