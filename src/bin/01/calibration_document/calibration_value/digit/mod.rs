pub mod string_digit;

use string_digit::StringDigit;

pub enum Digit {
    NumericalDigit(u32),
    StringDigit(StringDigit),
}

impl Digit {
    pub fn into_value(self) -> u32 {
        match self {
            Digit::NumericalDigit(digit) => digit,
            Digit::StringDigit(digit) => digit as u32,
        }
    }
}
