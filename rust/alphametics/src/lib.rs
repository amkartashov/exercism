use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = AlphameticPuzzle::from(input);
    puzzle.find_solution2()
}

#[derive(Clone, Debug)]
struct AlphameticPuzzle {
    leading_letters: HashSet<char>,
    even_letters: HashSet<char>,
    summands: AlphameticNumbersSum,
    sum: AlphameticNumber,
    letters: Vec<char>,
}

impl AlphameticPuzzle {
    fn from(expression: &str) -> Self {
        // "AA + BB + DD == CC" -> "AA", "BB", "DD", "CC"
        let mut numbers = expression
            .split(|c: char| !c.is_alphabetic())
            .filter(|&str| str.len() > 0)
            .map(|str| str.chars().into_iter().rev().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let maxlen = numbers.iter().map(|str| str.len()).max().unwrap();

        let mut unique_letters = HashSet::new();
        let mut letters = Vec::new();
        let mut leading_letters = HashSet::new();

        // we enumberate letters starting from units column
        // so to fail (later) on wrong permutation earlier
        for idx in 0..maxlen {
            for num in &numbers {
                if let Some(letter) = num.get(idx) {
                    if !unique_letters.contains(letter) {
                        unique_letters.insert(*letter);
                        letters.push(*letter);
                    }
                }
            }
        }

        // find all first letters
        // later we will use them for discarding permutations
        // with leading zeroes
        for num in &numbers {
            leading_letters.insert(num.last().copied().unwrap());
        }

        // separate the last number as sum
        let sum_number = numbers.pop().unwrap();
        let sum = AlphameticNumber::from(sum_number);

        // prepare summands
        let mut summands = AlphameticNumbersSum::new();
        for num in numbers {
            let num = AlphameticNumber::from(num);
            summands += num;
        }

        // find all letters which are supposed to be even
        let mut even_letters = HashSet::new();

        for (idx, letters_count) in summands.letters_count.iter().enumerate() {
            if letters_count.iter().all(|(_, &count)| count % 2 == 0) {
                even_letters.insert(sum.letters[idx]);
            }
        }

        summands.convert_to_columns();

        dbg!(AlphameticPuzzle {
            leading_letters,
            even_letters,
            summands,
            sum,
            letters,
        })
    }

    fn find_solution2(&self) -> Option<HashMap<char, u8>> {
        let mut solution = HashMap::with_capacity(self.letters.len());
        let mut used_digits = HashSet::with_capacity(self.letters.len());

        self.find_solution2_recursive(&mut solution, &mut used_digits)
    }

    fn find_solution2_recursive(
        &self,
        solution: &mut HashMap<char, u8>,
        used_digits: &mut HashSet<u8>,
    ) -> Option<HashMap<char, u8>> {
        if solution.len() >= self.letters.len() {
            if self.test_solution(solution) {
                return Some(solution.clone());
            } else {
                return None;
            }
        }

        let letter = self.letters[solution.len()];

        for digit in 0..10u8 {
            if used_digits.contains(&digit) {
                continue;
            }

            if digit == 0 && self.leading_letters.contains(&letter) {
                continue;
            }

            if digit % 2 == 1 && self.even_letters.contains(&letter) {
                continue;
            }

            solution.insert(letter, digit);
            used_digits.insert(digit);

            if !self.find_solution2_partial_test_ok(solution) {
                solution.remove(&letter);
                used_digits.remove(&digit);
                continue;
            }

            if let Some(solution) = self.find_solution2_recursive(solution, used_digits) {
                return Some(solution);
            }

            solution.remove(&letter);
            used_digits.remove(&digit);
        }

        None
    }

    fn find_solution2_partial_test_ok(&self, solution: &HashMap<char, u8>) -> bool {
        let mut shift = 0;

        for idx in 0..self.sum.letters.len() {
            if let Some(&sum) = solution.get(&self.sum.letters[idx]) {
                if idx == self.summands.letters_count.len() {
                    if sum > idx as u8 {
                        return false;
                    }
                    return true;
                }

                let mut rhs_sum = 0;

                for (letter, count) in self.summands.letters_columns[idx].iter() {
                    if let Some(&letter_value) = solution.get(letter) {
                        rhs_sum += *count as u64 * letter_value as u64;
                    } else {
                        return true;
                    }
                }

                rhs_sum += shift;

                if (sum % 10) != (rhs_sum % 10) as u8 {
                    return false; ///////   This is what we looking for to eliminate unwanted permutation
                }

                shift = rhs_sum / 10;
            } else {
                return true;
            }
        }

        return true;
    }

    fn test_solution(&self, solution: &HashMap<char, u8>) -> bool {
        self.sum.value(solution) == self.summands.value(solution)
    }
}

#[derive(Clone, Debug)]
struct AlphameticNumber {
    // can be single number or sum of numbers
    // ABC ->
    //      letters: [C, B, A]
    letters: Vec<char>,
}

impl AlphameticNumber {
    fn from(letters: Vec<char>) -> Self {
        AlphameticNumber { letters }
    }

    fn value(&self, solution: &HashMap<char, u8>) -> u64 {
        let mut sum = 0;

        for l in self.letters.iter().rev() {
            let val = *solution.get(l).unwrap() as u64;
            sum *= 10;
            sum += val;
        }

        sum
    }
}

#[derive(Clone, Debug)]
struct AlphameticNumbersSum {
    // ABC + AED + CBBB ->
    // [(B->1,C->1,D->1), (B->2,E->1), (A->2,B->1), (C->1)]
    letters_count: Vec<HashMap<char, u8>>,
    letters_columns: Vec<Vec<(char, u8)>>,
}

impl AlphameticNumbersSum {
    fn new() -> Self {
        Self {
            letters_count: Vec::new(),
            letters_columns: Vec::new(),
        }
    }

    fn convert_to_columns(&mut self) {
        for lc in &self.letters_count {
            let mut l_c_vec = Vec::new();
            l_c_vec.extend(lc.clone());
            self.letters_columns.push(l_c_vec);
        }
    }

    fn value(&self, solution: &HashMap<char, u8>) -> u64 {
        let mut sum = 0;

        for lc in self.letters_columns.iter().rev() {
            sum *= 10;
            sum += lc
                .iter()
                .map(|(letter, count)| (*solution.get(letter).unwrap() as u64) * (*count as u64))
                .sum::<u64>();
        }
        sum
    }
}

impl std::ops::AddAssign<AlphameticNumber> for AlphameticNumbersSum {
    fn add_assign(&mut self, rhs: AlphameticNumber) {
        if rhs.letters.len() > self.letters_count.len() {
            self.letters_count.resize(rhs.letters.len(), HashMap::new());
        }
        for (idx, rhs_letter) in rhs.letters.iter().enumerate() {
            let entry = self.letters_count[idx].entry(*rhs_letter).or_default();
            *entry += 1;
        }
    }
}
