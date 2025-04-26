pub mod iterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

// Convert integer to RomanNumber
impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let mut digits = Vec::new();
        let values = [1000, 500, 100, 50, 10, 5, 1];

        for &v in values.iter() {
            while value >= v {
                digits.push(RomanDigit::from(v));
                value -= v;
            }
        }

        RomanNumber(digits)
    }
}

// Convert integer to RomanDigit
impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid RomanDigit value"),
        }
    }
}

// Add .next() directly for RomanNumber
impl RomanNumber {
    pub fn next(&self) -> Option<RomanNumber> {
        iterator::next_roman(self)
    }
}
