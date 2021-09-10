use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: BTreeSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let mut factors = BTreeSet::new();
        factors.insert((a, b));
        let value = a * b;

        Palindrome { value, factors }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.insert((a, b));
    }

    pub fn update_factors(&mut self, min: u64, max: u64) {
        let &(a, b) = self.factors.iter().next().unwrap();
        let primes: Vec<_> = GetPrimeFactors::new(a)
            .chain(GetPrimeFactors::new(b))
            .collect();
        (0..primes.len())
            .map(|split_idx| {
                let (factors1, factors2) = primes.split_at(split_idx);
                (
                    factors1.iter().fold(1, |acc, &f| acc * f),
                    factors2.iter().fold(1, |acc, &f| acc * f),
                )
            })
            .filter(|&(f1, f2)| min <= f1 && min <= f2 && f1 <= max && f2 <= max)
            .for_each(|pair| {
                self.factors.insert(pair);
            });
    }

    pub fn is_valid(&self) -> bool {
        if self.value < 10 {
            return true;
        }

        let mut tmp = self.value;
        let mut reversed = 0;

        while tmp > 0 {
            reversed = reversed * 10 + tmp % 10;
            tmp /= 10;
        }

        self.value == reversed
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if max < min {
        return None;
    }

    let diff = max - min;

    // search for minimal palindrome
    // start from (a,b) = (min,min)
    // iterate over pairs with constant sum: a+b = 2*min + inc, inc = 0..2*diff
    // start from maximum difference between a,b to have minimal product
    let mut miniter = (0..=2 * diff)
        .flat_map(|inc| (0..=(inc / 2)).rev().map(move |d| (min + inc - d, min + d)))
        .filter(|&(a, b)| a <= max && b <= max)
        .map(|(a, b)| Palindrome::new(a, b));

    // search for maximum palindrome
    // start from (a,b) = (max,mac)
    // iterate over pairs with constant sum: a+b = 2*max - dec, dec = 0..2*diff
    // start from minimum difference between a,b to maximize product
    let mut maxiter = (0..=2 * diff)
        .flat_map(|dec| (0..=(dec / 2)).map(move |i| (max - dec + i, max - i)))
        .filter(|&(a, b)| a >= min && b >= min)
        .map(|(a, b)| Palindrome::new(a, b));

    if let Some(mut pmin) = miniter.find(Palindrome::is_valid) {
        let mut pmax = maxiter.find(Palindrome::is_valid).unwrap();

        pmin.update_factors(min, max);
        if pmin.value == pmax.value {
            pmax.factors = pmin.factors.clone()
        } else {
            pmax.update_factors(min, max);
        }

        return Some((pmin, pmax));
    }

    return None;
}

struct GetPrimeFactors {
    next_factor: u64,
    n: u64,
}

impl GetPrimeFactors {
    fn new(n: u64) -> Self {
        Self { next_factor: 2, n }
    }
}

impl Iterator for GetPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for d in self.next_factor..=self.n {
            if self.n % d == 0 {
                self.n /= d;
                self.next_factor = d;
                return Some(d);
            }
        }

        None
    }
}
