#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        Some(match num.cmp(&Factors::new(num).sum()) {
            std::cmp::Ordering::Less => Classification::Abundant,
            std::cmp::Ordering::Equal => Classification::Perfect,
            std::cmp::Ordering::Greater => Classification::Deficient,
        })
    }
}

struct Factors {
    next_factor: u64,
    n: u64,
}

impl Factors {
    fn new(n: u64) -> Self {
        Self { next_factor: 1, n }
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for d in self.next_factor..=(self.n / 2) {
            if self.n % d == 0 {
                self.next_factor = d + 1;
                return Some(d);
            }
        }

        None
    }
}
