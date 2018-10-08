pub fn reply(message: &str) -> &str {
    let last_char = match message.trim().chars().last() {
        Some(c) => c,
        None => return "Fine. Be that way!",
    };
    if message.to_uppercase() == message && message.chars().any(char::is_alphabetic) {
        match last_char {
            '?' => "Calm down, I know what I'm doing!",
            _ => "Whoa, chill out!",
        }
    } else {
        match last_char {
            '?' => "Sure.",
            _ => "Whatever.",
        }
    }
}
