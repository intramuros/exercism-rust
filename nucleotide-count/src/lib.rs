use std::collections::HashMap;

pub fn check_nucleotide(nucleotide: char) -> Result<char, char> {
    match nucleotide {
        'A' | 'T' | 'C' | 'G' => Ok(nucleotide),
        _ => Err(nucleotide),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    check_nucleotide(nucleotide)?;
    dna.chars().try_fold(0, |acc, n| match n {
        _ if nucleotide == n => Ok(acc + 1),
        'A' | 'T' | 'C' | 'G' => Ok(acc),
        _ => Err(n),
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    result.insert('A', 0);
    result.insert('T', 0);
    result.insert('C', 0);
    result.insert('G', 0);

    for c in dna.chars() {
        check_nucleotide(c)?;
        result.entry(c).and_modify(|e| *e += 1);
    }
    Ok(result)
}
