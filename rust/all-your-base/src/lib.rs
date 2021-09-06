#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let num = base_number::Number::from_digits(from_base, digits)?;
    let converted = num.convert(to_base)?;
    Ok(converted.digits())
}

mod base_number {

    #[derive(Clone, Debug)]
    pub struct Number {
        base: u32,

        // little-endian.
        // F.e. 1234 will be stored as vec![4,3,2,1]
        digits: Vec<u32>,
    }

    impl std::ops::Add<&'_ Self> for Number {
        type Output = Self;

        fn add(mut self, other: &'_ Number) -> Number {
            if self.base == other.base {
                self.add_same_base(&other);
            } else {
                self.add_same_base(&other.convert(self.base).unwrap());
            }

            self
        }
    }

    impl std::ops::AddAssign<&'_ Self> for Number {
        fn add_assign(&mut self, rhs: &'_ Self) {
            if self.base == rhs.base {
                self.add_same_base(&rhs);
            } else {
                self.add_same_base(&rhs.convert(self.base).unwrap());
            }
        }
    }

    impl std::ops::MulAssign<u32> for Number {
        fn mul_assign(&mut self, rhs: u32) {
            self.multiply_by_u32(rhs);
        }
    }

    impl std::ops::Mul<u32> for Number {
        type Output = Self;

        fn mul(mut self, rhs: u32) -> Self {
            self.multiply_by_u32(rhs);
            self
        }
    }

    impl std::ops::MulAssign<&'_ Self> for Number {
        fn mul_assign(&mut self, rhs: &'_ Self) {
            if self.base == rhs.base {
                self.multiply_same_base(&rhs);
            } else {
                self.multiply_same_base(&rhs.convert(self.base).unwrap());
            }
        }
    }

    impl Number {
        pub fn new(base: u32) -> Result<Self, super::Error> {
            if base < 2 {
                return Err(super::Error::InvalidInputBase);
            };

            Ok(Number {
                base,
                digits: vec![0],
            })
        }

        pub fn from_u32(base: u32, number: u32) -> Result<Self, super::Error> {
            if base < 2 {
                return Err(super::Error::InvalidInputBase);
            };

            Ok(Number {
                base,
                digits: Number::to_digits(number, base),
            })
        }

        pub fn from_digits(base: u32, digits: &[u32]) -> Result<Self, super::Error> {
            if base < 2 {
                return Err(super::Error::InvalidInputBase);
            };

            for &x in digits {
                if x >= base {
                    return Err(super::Error::InvalidDigit(x));
                }
            }

            let mut n = Number {
                base,
                digits: digits.iter().rev().copied().collect(),
            };

            n.drop_zeros();

            Ok(n)
        }

        pub fn convert(&self, to_base: u32) -> Result<Self, super::Error> {
            if to_base < 2 {
                return Err(super::Error::InvalidOutputBase);
            };

            if self.base == to_base {
                return Ok(self.clone());
            }

            let mut result = Number::new(to_base).unwrap();

            // self.base as Number with base = to_base
            let base_in_dest_base = Number::from_u32(to_base, self.base).unwrap();

            // powers of self.base (as Number with base = to_base), starting from 0:
            // 1, base, base^2, base^3,...
            let base_powers = (0..).scan(Number::from_u32(to_base, 1).unwrap(), |pow, _| {
                let res = pow.clone();
                *pow *= &base_in_dest_base;
                Some(res)
            });

            for (&digit, base_power) in self.digits.iter().zip(base_powers) {
                let digit_in_dest_base = base_power * digit;
                result += &digit_in_dest_base;
            }

            Ok(result)
        }

        pub fn digits(&self) -> Vec<u32> {
            let mut digits = self.digits.clone();
            digits.reverse();
            digits
        }

        fn add_same_base(&mut self, other: &Self) {
            assert!(self.base == other.base);
            self.raw_add_same_base(other);
            self.normalize();
        }

        fn multiply_by_power_of_base(&mut self, power: usize) {
            self.digits.resize(self.digits.len() + power, 0);
            self.digits.rotate_right(power);
        }

        fn multiply_by_u32(&mut self, multiplier: u32) {
            self.raw_multiply_by_u32(multiplier);
            self.normalize();
        }

        fn multiply_same_base(&mut self, other: &Self) {
            assert!(self.base == other.base);
            let mut result = self.raw_multiply_same_base(other);
            result.normalize();
            self.digits = result.digits;
        }

        /// truncate zeroes from the end (high digits)
        fn drop_zeros(&mut self) {
            if let Some(last_non_zero_position) = self.digits.iter().rposition(|&x| x > 0) {
                self.digits.resize(last_non_zero_position + 1, 0)
            } else {
                // all zeroes
                self.digits = vec![0];
                return;
            }
        }

        /// correct digits which are >= base
        fn normalize(&mut self) {
            self.drop_zeros();

            let mut shift = 0;
            for digit in &mut self.digits {
                let sum = shift + *digit;
                shift = sum / self.base;
                *digit = sum % self.base;
            }

            if shift > 0 {
                Number::to_digits(shift, self.base).iter().for_each(|&d| {
                    self.digits.push(d);
                })
            }
        }

        fn raw_multiply_by_u32(&mut self, multiplier: u32) {
            for digit in &mut self.digits {
                *digit *= multiplier;
            }
        }

        fn raw_multiply_same_base(&self, other: &Self) -> Number {
            let mut result = Number {
                base: self.base,
                digits: Vec::new(),
            };

            for (idx, &other_digit) in other.digits.iter().enumerate() {
                let mut copy = self.clone();
                copy.raw_multiply_by_u32(other_digit);
                copy.multiply_by_power_of_base(idx);
                result.raw_add_same_base(&copy);
            }

            result
        }

        fn raw_add_same_base(&mut self, other: &Self) {
            if other.digits.len() > self.digits.len() {
                self.digits.resize(other.digits.len(), 0);
            }

            for (idx, &other_digit) in other.digits.iter().enumerate() {
                self.digits[idx] += other_digit;
            }
        }

        fn to_digits(number: u32, base: u32) -> Vec<u32> {
            (0..)
                .scan(number, |number, _| {
                    if *number == 0 {
                        None
                    } else {
                        let digit = *number % base;
                        *number /= base;
                        Some(digit)
                    }
                })
                .collect()
        }
    }
}
