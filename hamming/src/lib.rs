/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len() == s2.len() {
        true => Some(s1.chars().zip(s2.chars()).fold(0, |acc, (x,y)| if x != y { acc + 1 } else { acc } )),
        false => None,
    }
}
