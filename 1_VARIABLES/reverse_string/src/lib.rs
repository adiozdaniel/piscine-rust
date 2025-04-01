// rev_str function takes a string slice and returns a new string with the characters in reverse order.
pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_str() {
        assert_eq!(rev_str("hello"), "olleh");
        assert_eq!(rev_str("Rust"), "tsuR");
        assert_eq!(rev_str(""), "");
        assert_eq!(rev_str("a"), "a");
        assert_eq!(rev_str("12345"), "54321");
    }
}
