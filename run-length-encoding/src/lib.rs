extern crate itertools;

use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| match group.count() {
            1 => c.to_string(),
            n => format!("{}{}", n, c),
        }).collect()
}

pub fn decode(source: &str) -> String {
    let mut final_string = String::new();
    let mut num_str = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            num_str.push(c);
        } else {
            final_string.push_str(&(c.to_string()).repeat(num_str.parse().unwrap_or(1)));
            num_str.clear();
        }
    }
    final_string
}
