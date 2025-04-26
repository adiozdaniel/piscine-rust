// find the first subword in a string and return it as a new string 
pub fn first_subword(mut s: String) -> String {
    // Convert the string to lowercase for easier processing

    // Find the index of the first lowercase letter or underscore
    let index = match s.chars().skip(1).position(|c| c.is_uppercase() || c == '_') {
        Some(idx) => idx+1,
        None => return s, // Return the original string if no sub-word is found
    };

    // Truncate the string up to the found index
    s.truncate(index);
    s
}

#[cfg(test)]
mod tests {
    use super::first_subword;

    #[test]
    fn single_word_lowercase() {
        assert_eq!(first_subword("hello".to_string()), "hello");
    }

    #[test]
    fn camel_case_word() {
        assert_eq!(first_subword("helloWorld".to_string()), "hello");
    }

    #[test]
    fn underscore_separated() {
        assert_eq!(first_subword("hello_world".to_string()), "hello");
    }

    #[test]
    fn pascal_case_word() {
        assert_eq!(first_subword("HelloWorld".to_string()), "Hello");
    }

    #[test]
    fn empty_string() {
        assert_eq!(first_subword("".to_string()), "");
    }

    #[test]
    fn all_uppercase() {
        assert_eq!(first_subword("HELLO".to_string()), "H");
    }

    #[test]
    fn mixed_characters() {
        assert_eq!(first_subword("hElLo".to_string()), "h");
    }

    #[test]
    fn stop_at_first_uppercase() {
        assert_eq!(first_subword("helloWorldAgain".to_string()), "hello");
    }

    #[test]
    fn stop_at_first_underscore() {
        assert_eq!(first_subword("hello_world_again".to_string()), "hello");
    }
}
