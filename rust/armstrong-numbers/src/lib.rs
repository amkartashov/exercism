pub fn is_armstrong_number(num: u32) -> bool {
    let digits = decimal_digits(num);
    num == digits
    .iter()
    .map(|&x| x.pow(digits.len() as u32))
    .sum()
}

fn decimal_digits(num: u32) -> Vec<u32> {
    num
    .to_string()
    .chars()
    .map(|c| c.to_string().parse().unwrap())
    .collect()
}