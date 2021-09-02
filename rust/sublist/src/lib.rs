#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_equal<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    first.len() == second.len()
        && first
            .iter()
            .zip(second)
            .all(|(elem1, elem2)| elem1 == elem2)
}

fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    if small.is_empty() {
        return true;
    }
    if small.len() >= big.len() {
        return false;
    }
    big.windows(small.len()).any(|w| is_equal(small, w))
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        std::cmp::Ordering::Equal => {
            if is_equal(first, second) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        std::cmp::Ordering::Greater => {
            if is_sublist(second, first) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        std::cmp::Ordering::Less => {
            if is_sublist(first, second) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}
