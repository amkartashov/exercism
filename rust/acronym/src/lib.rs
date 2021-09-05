fn abbreviate_camel_case(word: &str) -> Vec<char> {
    let mut iter = word.chars().enumerate().peekable();

    let mut uppers = Vec::new();

    while let Some((idx, char)) = iter.next() {
        if idx == 0 {
            uppers.extend(char.to_uppercase());
        } else if let Some(&(_, nextchar)) = iter.peek() {
            if char.is_uppercase() && nextchar.is_lowercase() {
                uppers.extend(char.to_uppercase());
            }
        }
    }

    uppers
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(|word| word.trim_matches('_'))
        .flat_map(|word| word.split('-'))
        .flat_map(|word| abbreviate_camel_case(word).into_iter())
        .collect()
}
