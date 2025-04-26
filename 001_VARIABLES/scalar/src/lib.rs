// returns the sum between two values from 0 to 255
pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

// returns the difference between two values from -32768 to 32767
pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

// returns the product of the multiplication between two values from -128 to 127
pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

// returns the quotient of the division between two 32bit values
pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

// returns the remainder of the division between two 32bit values
pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}
