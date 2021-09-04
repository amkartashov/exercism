/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut position = 0;

    // need to reverse because position is counted from right
    for n in code.chars().rev() {
        match n {
            // process numbers
            n if '0' <= n && n <= '9' => {
                position += 1;
                let mut n = n as u8 - '0' as u8;
                // every second one (from right) is doubled
                if position % 2 == 0 {
                    n = 2 * n;
                    if n > 9 {
                        n = n - 9;
                    }
                }
                sum += n;
            }

            // spaces are ignored
            n if n.is_whitespace() => {}

            // any other character makes number invalid
            _ => return false,
        };
    }

    // at least 2 numbers
    // sum should be divisible by 10
    position > 1 && sum % 10 == 0
}
