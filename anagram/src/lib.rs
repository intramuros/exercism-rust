use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter_map(|elem| {
            if word.to_lowercase() != elem.to_lowercase()
                && lowercase_sort(word) == lowercase_sort(elem)
            {
                Some(*elem)
            } else {
                None
            }
        })
        .collect()
}

fn lowercase_sort(s: &str) -> String {
    let mut cloned: Vec<char> = s.clone().to_lowercase().chars().collect();
    cloned.sort();
    cloned.iter().collect()
}
