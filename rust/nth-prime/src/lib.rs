pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut cache = vec![2];

    let mut current_prime = *cache.last().unwrap();
    let mut next_prime: u32;
    let mut check_edge = 0;

    while cache.len() < n as usize + 1 {
        next_prime = ((current_prime + 1)..)
            .filter(|&candidate| {
                check_edge += cache[check_edge..]
                    .iter()
                    .enumerate()
                    .find(|&(_, &d)| d > (candidate as f64).sqrt() as u32 - 1)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                cache[..=check_edge].iter().all(|&d| candidate % d != 0)
            })
            .next()
            .unwrap();

        cache.push(next_prime);
        current_prime = next_prime;
    }

    cache[n as usize]
}
