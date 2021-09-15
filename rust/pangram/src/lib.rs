use std::{collections::BTreeSet, iter::FromIterator};

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    const ABC: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut abc_hash = BTreeSet::from_iter(&ABC);
    for c in sentence.chars().map(|c| c.to_ascii_lowercase()) {
        abc_hash.remove(&c);
        if abc_hash.is_empty() {
            return true;
        }
    }
    
    false
}
