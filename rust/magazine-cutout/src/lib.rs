// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

fn count_words<'a>(text: &'a [&str]) -> HashMap<&'a str, u32>{
    let mut words = HashMap::new();
    for &w in text {
        *words.entry(w).or_insert(0) += 1;
    }
    words
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = count_words(magazine);
    let note_words = count_words(note);
    note_words
    .iter()
    .all(|(&word, &count)| count <= *magazine_words.get(word).unwrap_or(&0))
}
