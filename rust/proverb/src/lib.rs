pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let first = list[0];
    let rest = &list[1..];

    let mut result = list
        .iter()
        .zip(rest.iter())
        .map(|(&f, &s)| format!("For want of a {} the {} was lost.", f, s))
        .collect::<Vec<_>>();

    result.push(format!("And all for the want of a {}.", first));

    result.join("\n")
}
