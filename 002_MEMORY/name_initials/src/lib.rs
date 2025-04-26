// Create a function that takes a vector of names and returns a vector of initials. The initials should be in uppercase and separated by a period and a space. If a name has more than one word, only the first letter of each word should be used. If a name is empty or contains only whitespace, the result should be an empty string.
pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|s| s.chars().next().map(|c| format!("{c}.")))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::initials;

    #[test]
    fn single_name_single_word() {
        assert_eq!(initials(vec!["John"]), vec!["J."]);
    }

    #[test]
    fn single_name_multiple_words() {
        assert_eq!(initials(vec!["John Doe"]), vec!["J. D."]);
    }

    #[test]
    fn multiple_names() {
        assert_eq!(
            initials(vec!["John Doe", "Jane Smith"]),
            vec!["J. D.", "J. S."]
        );
    }

    #[test]
    fn names_with_multiple_spaces() {
        assert_eq!(
            initials(vec!["John   Michael  Doe"]),
            vec!["J. M. D."]
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(initials(vec![""]), vec![""]);
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(initials(vec!["   "]), vec![""]);
    }

    #[test]
    fn unicode_names() {
        assert_eq!(
            initials(vec!["こんにちは World", "🦀 Rustacean"]),
            vec!["こ. W.", "🦀. R."]
        );
    }

    #[test]
    fn mixed_case_names() {
        assert_eq!(
            initials(vec!["jOHN dOE", "aNNIE mAY"]),
            vec!["j. d.", "a. m."]
        );
    }

    #[test]
    fn single_character_names() {
        assert_eq!(
            initials(vec!["A B C", "X Y Z"]),
            vec!["A. B. C.", "X. Y. Z."]
        );
    }

    #[test]
    fn empty_input_vector() {
        assert_eq!(initials(vec![]), vec![] as Vec<String>);
    }
}
