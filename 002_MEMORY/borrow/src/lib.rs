pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::str_len;

    #[test]
    fn empty_string() {
        assert_eq!(str_len(""), 0);
    }

    #[test]
    fn ascii_string() {
        assert_eq!(str_len("hello"), 5);
    }

    #[test]
    fn string_with_spaces() {
        assert_eq!(str_len("hello world"), 11);
    }

    #[test]
    fn string_with_special_chars() {
        assert_eq!(str_len("!@#$%^&*()"), 10);
    }

    #[test]
    fn string_with_mixed_chars() {
        assert_eq!(str_len("Rust 🦀 is cool!"), 15); // Note: crab emoji is 4 bytes
    }
}
