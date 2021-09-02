use std::collections::{HashMap, HashSet};

fn count_letters(word: &String) -> HashMap<char, u32> {
    let mut letters = HashMap::new();
    for c in word.chars() {
        *letters.entry(c).or_insert(0) += 1;
    }
    letters
}

/// Note: letters and word should be in lowercase!
fn is_anagram(letters: &HashMap<char, u32>, anagram: String) -> bool {
    let mut anagram_letters = letters.clone();

    for c in anagram.chars() {
        if let Some(left) = anagram_letters.get_mut(&c) {
            if *left == 0 {
                return false;
            } else {
                *left -= 1;
            }
        } else {
            // no such character in letters
            return false;
        }
    }

    // all counting should end at 0, otherwise there are less letters in word
    anagram_letters.values().all(|&count| count == 0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let letters = count_letters(&word);
    possible_anagrams
        .iter()
        .filter(|&&a| {
            let a = a.to_lowercase();
            a != word && is_anagram(&letters, a)
        })
        .map(|&a| a)
        .collect()
}
