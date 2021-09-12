pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    const VOWELS_CLUSTERS: [&str; 2] = ["xr", "yt"];

    // word starts with vowel
    if word.starts_with(&VOWELS[..]) || VOWELS_CLUSTERS.iter().any(|cl| word.starts_with(cl)) {
        return String::from(word) + "ay";
    }

    // find starting consonant
    let vowel_pos = word.find(&VOWELS[..]).unwrap_or_else(|| word.len());
    let (consonant, rest) = &word.split_at(vowel_pos);

    // y at the end of consonant is vowel
    if let Some(y_pos) = consonant.find('y') {
        // not the first
        if y_pos > 0 {
            // find position in word
            let y_pos = word.find('y').unwrap();
            let (consonant, rest) = &word.split_at(y_pos);
            return rest.to_string() + consonant + "ay";
        }
    }

    if consonant.len() == word.len() {
        return word.to_string() + "ay";
    }

    // consonant followed by `qu`
    if consonant.chars().next_back().unwrap() == 'q' && rest.chars().next().unwrap() == 'u' {
        return rest[1..].to_string() + consonant + "u" + "ay";
    }

    rest.to_string() + consonant + "ay"
}
