use crossbeam;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    fn frequency_worker(input: &[&str]) -> HashMap<char, usize> {
        input
            .iter()
            .flat_map(|s| s.chars())
            .filter(|c| c.is_alphabetic())
            .flat_map(char::to_lowercase)
            .fold(HashMap::new(), |mut m, c| {
                let entry = m.entry(c).or_default();
                *entry += 1;
                m
            })
    }

    let mut chunk_size = input.len() / worker_count;
    if chunk_size == 0 {
        chunk_size = 1;
    }

    crossbeam::scope(|s| {
        let threads: Vec<_> = input
            .chunks(chunk_size)
            .map(|input| s.spawn(move |_| frequency_worker(input)))
            .collect();

        threads
            .into_iter()
            .fold(HashMap::new(), |mut totals, thread| {
                let m = thread.join().unwrap();
                for (c, f) in m.iter() {
                    let entry = totals.entry(*c).or_default();
                    *entry += f;
                }
                totals
            })
    })
    .unwrap()
}
