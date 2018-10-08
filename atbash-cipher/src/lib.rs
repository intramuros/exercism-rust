use std::char;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded = plain
        .chars()
        .filter(|s| s.is_ascii())
        .filter_map(|s| match s {
            '0'...'9' => Some(s),
            'a'...'z' | 'A'...'Z' => s
                .to_digit(36)
                .and_then(|num| char::from_digit(35 - num + 10, 36)),
            _ => None,
        }).collect::<Vec<_>>();
    let mut final_string = String::new();
    for (i, &x) in encoded.iter().enumerate() {
        if i % 5 == 0 && i != 0 {
            final_string.push(' ');
        }
        final_string.push(x);
    }
    final_string
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|s| s.is_ascii())
        .filter_map(|s| match s {
            '0'...'9' => Some(s),
            'a'...'z' | 'A'...'Z' => s
                .to_digit(36)
                .and_then(|num| char::from_digit(35 - num + 10, 36)),
            _ => None,
        }).collect::<String>()
}
