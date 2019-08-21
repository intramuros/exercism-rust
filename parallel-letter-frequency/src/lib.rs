use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut letters = HashMap::new();
    let chunk_size = match input.len() / worker_count {
        0 if input.is_empty() => return letters,
        0 => input.len(),
        s => s,
    };

    let mut thread_handles = vec![];

    for piece in input.chunks(chunk_size) {
        let owned: String = piece.to_owned().join("");
        thread_handles.push(thread::spawn(move || count_letters(owned)));
    }

    for handle in thread_handles {
        if let Ok(thread_result) = handle.join() {
            for (key, value) in thread_result {
                (*letters.entry(key).or_insert(0)) += value;
            }
        }
    }
    letters
}

fn count_letters(input: String) -> HashMap<char, usize> {
    let mut letters = HashMap::new();
    for ch in input.chars() {
        if ch.is_alphabetic() {
            let counter = letters
                .entry(ch.to_lowercase().next().unwrap())
                .or_insert(0);
            *counter += 1;
        }
    }
    letters
}
