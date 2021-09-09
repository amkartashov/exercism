use std::{collections::HashMap, iter::FromIterator};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {}
        x => return Err(x),
    };

    let mut sum = 0;
    for c in dna.chars() {
        match c {
            c if c == nucleotide => sum += 1,
            'A' | 'C' | 'G' | 'T' => {}
            x => return Err(x),
        }
    }

    Ok(sum)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from_iter([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => *result.get_mut(&c).unwrap() += 1,
            x => return Err(x),
        }
    }

    Ok(result)
}
