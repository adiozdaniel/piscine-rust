// returns the factorial of a given number
pub fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}

// This is the test of the project
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }
}
