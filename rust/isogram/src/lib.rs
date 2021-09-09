use std::collections::BTreeSet;

pub fn check(candidate: &str) -> bool {
    let mut chars = BTreeSet::new();
    for c in candidate
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .flat_map(char::to_lowercase)
    {
        if chars.contains(&c) {
            return false;
        };
        chars.insert(c);
    }

    true
}
