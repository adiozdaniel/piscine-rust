// This is the library crate for the division_and_remainder crate.
pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

// Path: division_and_remainder/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 3), (3, 1));
        assert_eq!(divide(20, 4), (5, 0));
        assert_eq!(divide(-10, 3), (-3, -1));
        assert_eq!(divide(10, -3), (-3, 1));
        assert_eq!(divide(-10, -3), (3, -1));
    }
}
