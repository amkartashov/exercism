pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
        .chain(
            list.first()
                .iter()
                .map(|&&s| format!("And all for the want of a {}.", s)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
