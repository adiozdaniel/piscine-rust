use crate::{RomanDigit, RomanNumber};

pub fn next_roman(number: &RomanNumber) -> Option<RomanNumber> {
    // Convert RomanNumber to integer
    let mut value = roman_to_integer(&number.0);

    // Increment by 1
    value += 1;

    // If value is zero (very rare here), return None
    if value == 0 {
        None
    } else {
        Some(RomanNumber::from(value))
    }
}

// Helper function: RomanDigit vector to integer
fn roman_to_integer(digits: &[RomanDigit]) -> u32 {
    let mut value = 0;
    for digit in digits {
        value += match digit {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }
    }
    value
}
