use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::swap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = AlphameticPuzzle::from(input);
    puzzle.find_solution()
}

#[derive(Clone, Debug)]
struct AlphameticPuzzle {
    addends: AlphameticNumber,
    sum: AlphameticNumber,
    letters: Vec<char>,
}

impl AlphameticPuzzle {
    fn from(expression: &str) -> Self {
        // "AA + BB + DD == CC" -> "AA + BB + DD ", " CC"
        let mut split_by_eq = expression.split("==");

        AlphameticPuzzle {
            addends: split_by_eq
                .next()
                .unwrap()
                .split('+')
                .map(|number| AlphameticNumber::from(number.trim()))
                .fold(AlphameticNumber::new(), |mut a, b| {
                    a += b;
                    a
                }),
            sum: AlphameticNumber::from(split_by_eq.next().unwrap().trim()),
            letters: AlphameticPuzzle::letters(expression),
        }
    }

    fn find_solution(&self) -> Option<HashMap<char, u8>> {
        let mut permutations = DigitPermutations::new(self.letters.len() as u8);

        while let Some(permutation) = permutations.next() {
            let solution = self
                .letters
                .iter()
                .copied()
                .zip(permutation.iter().copied())
                .collect();

            if self.test_solution(&solution) {
                return Some(solution);
            }
        }

        None
    }

    fn test_solution(&self, solution: &HashMap<char, u8>) -> bool {
        if let (Some(sum), Some(addends)) = (self.sum.value(solution), self.addends.value(solution))
        {
            sum == addends
        } else {
            false
        }
    }

    fn letters(expression: &str) -> Vec<char> {
        let mut letters = HashSet::new();
        for c in expression.chars().filter(|&c| c.is_alphabetic()) {
            letters.insert(c);
        }
        letters.iter().copied().collect()
    }
}

#[derive(Clone, Debug)]
struct AlphameticNumber {
    // can be single number or sum of numbers
    // ABC ->
    //      first_digits: (A)
    //      number: [(C->1), (B->1), (A->1)]
    // ABC + AED + CBBB ->
    //      first_digits: (A, C)
    //      number: [(B->1,C->1,D->1), (B->2,E->1), (A->2,B->1), (C->1)]
    first_digits: HashSet<char>,
    number: Vec<HashMap<char, u8>>,
}

impl AlphameticNumber {
    fn new() -> Self {
        AlphameticNumber::from(&"")
    }

    fn from(string: &str) -> Self {
        let mut first_digits = HashSet::new();
        if let Some(first) = string.chars().next() {
            first_digits.insert(first);
        }
        let mut number = Vec::new();
        for c in string.chars().rev() {
            let mut c_map = HashMap::new();
            c_map.insert(c, 1);
            number.push(c_map);
        }

        AlphameticNumber {
            first_digits,
            number,
        }
    }

    fn value(&self, solution: &HashMap<char, u8>) -> Option<u64> {
        // no leading zeroes are allowed
        for digit in &self.first_digits {
            if solution.get(&digit) == Some(&0) {
                return None;
            }
        }

        let mut tens = 1;

        Some(
            self.number
                .iter()
                .map(|cn| {
                    let mut sum = 0;
                    for (c, &n) in cn {
                        let c = *solution.get(&c).unwrap();
                        sum += (n as u64) * (c as u64);
                    }
                    sum *= tens;
                    tens *= 10;

                    sum
                })
                .sum(),
        )
    }
}

impl std::ops::AddAssign<Self> for AlphameticNumber {
    fn add_assign(&mut self, mut rhs: Self) {
        if self.first_digits.len() < rhs.first_digits.len() {
            swap(&mut self.first_digits, &mut rhs.first_digits)
        }
        for digit in rhs.first_digits {
            self.first_digits.insert(digit);
        }

        if self.number.len() < rhs.number.len() {
            swap(&mut self.number, &mut rhs.number)
        }

        for (self_cn, rhs_cn) in self.number.iter_mut().zip(rhs.number.iter_mut()) {
            if self_cn.len() < rhs_cn.len() {
                swap(self_cn, rhs_cn)
            }
            for (&letter, count) in rhs_cn {
                let entry = self_cn.entry(letter).or_default();
                *entry += *count
            }
        }
    }
}

/// quasi-iterator other all possible permutations of n unique digits (0 to 9)
#[derive(Clone, Debug)]
struct DigitPermutations {
    n: u8,
    state: Vec<u8>,
}

impl DigitPermutations {
    fn new(n: u8) -> Self {
        Self {
            n,
            state: Vec::with_capacity(n as usize),
        }
    }

    fn next(&mut self) -> Option<&Vec<u8>> {
        if self.state.is_empty() {
            if self.n == 0 || self.n > 10 {
                return None;
            }
            // we start with 0,1,2,3,...
            self.state.extend(0..self.n);
            return Some(&self.state);
        }

        let mut increased_flag = false;

        // We iterate over all permutations by trying to increase the latest digits first, that's why rev() is here
        for idx in (0..self.n as usize).rev() {
            let current_digit = self.state[idx];
            // we choose the next possible digit for current[idx] which is not already in use in current[0..idx]
            if let Some(next_digit) = ((current_digit + 1)..=9)
                .filter(|&digit| (0..idx).all(|i| self.state[i] != digit))
                .next()
            {
                self.state[idx] = next_digit;
                // reset digits after this position (idx) anew
                for i in (idx + 1)..self.n as usize {
                    self.state[i] = (0..=9)
                        .filter(|&digit| (0..i).all(|j| self.state[j] != digit))
                        .next()
                        .unwrap()
                }
                increased_flag = true;
                break;
            }
        }
        // if we were not able to increase, this means that we reached maximum 9,8,7,6,..
        if !increased_flag {
            return None;
        } else {
            return Some(&self.state);
        }
    }
}
