/// Determines whether the supplied string is a valid ISBN number
/// The ISBN-10 format is 9 digits (0 to 9) plus one check character (either a digit or an X only). In the case the check character is an X, this represents the value '10'. These may be communicated with or without hyphens, and can be checked for their validity by the following formula:
///
/// (x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0
///
/// If the result is 0, then it is a valid ISBN-10, otherwise it is invalid.
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut size_is_10 = false;
    let mut sum = 0;

    for (i, c) in isbn.chars().filter(|c| *c != '-').rev().enumerate() {
        let i = i + 1;

        if i > 10 {
            return false;
        }

        if i == 10 {
            size_is_10 = true
        };

        sum += match c {
            'X' if i == 1 => 10,
            '0'..='9' => (c as usize - '0' as usize) * i,
            _ => return false,
        }
    }

    size_is_10 && sum % 11 == 0
}
