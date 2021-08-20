pub fn factors(n: u64) -> Vec<u64> {
    return rec_factors(Vec::new(), n);
}

fn rec_factors(mut factors: Vec<u64>, n: u64) -> Vec<u64> {
    for d in 2..((n as f64).sqrt() as u64 + 1) {
        if n % d == 0 {
            factors.push(d);
            return rec_factors(factors, n / d);
        }
    }
    if n != 1 {
        factors.push(n)
    }
    factors
}
