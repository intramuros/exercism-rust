pub fn check(candidate: &str) -> bool {
    !candidate
        .to_lowercase()
        .chars()
        .enumerate()
        .skip(1)
        .any(|(i, x)| x.is_alphabetic() && candidate[0..i].to_lowercase().contains(x))
}
