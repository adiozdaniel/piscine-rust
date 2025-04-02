// This is the library crate for the doubtful project. It contains the function that appends a question mark to a string.
pub fn doubtful(s: &mut String ) {
	s.push('?')
}

#[cfg(test)]
mod tests {
    use super::doubtful;

    #[test]
    fn appends_question_mark_to_empty_string() {
        let mut s = String::new();
        doubtful(&mut s);
        assert_eq!(s, "?");
    }

    #[test]
    fn appends_question_mark_to_non_empty_string() {
        let mut s = String::from("Hello");
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }

    #[test]
    fn adds_question_mark_to_string_that_already_has_one() {
        let mut s = String::from("What?");
        doubtful(&mut s);
        assert_eq!(s, "What??");
    }

    #[test]
    fn works_with_unicode_characters() {
        let mut s = String::from("Привет");
        doubtful(&mut s);
        assert_eq!(s, "Привет?");
    }

    #[test]
    fn adds_only_one_question_mark() {
        let mut s = String::from("Test");
        doubtful(&mut s);
        assert_eq!(s.len(), 5); // "Test?" is 5 bytes
        assert_eq!(s.chars().last(), Some('?'));
    }
}
