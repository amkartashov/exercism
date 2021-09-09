use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(score, chars)| {
            chars
                .iter()
                .map(move |char| (char.to_lowercase().next().unwrap(), *score))
        })
        .collect()
}
