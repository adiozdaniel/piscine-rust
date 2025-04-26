// Define a public function `score` that takes a string slice `word` and returns a u64
pub fn score(word: &str) -> u64 {
    word.chars() // Iterate over each character in the string
        .map(|x| match x.to_ascii_lowercase() { // Convert character to lowercase and match it
            // Characters worth 1 point
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            // Characters worth 2 points
            'd' | 'g' => 2,
            // Characters worth 3 points
            'b' | 'c' | 'm' | 'p' => 3,
            // Characters worth 4 points
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            // Character worth 5 points
            'k' => 5,
            // Characters worth 8 points
            'j' | 'x' => 8,
            // Characters worth 10 points
            'q' | 'z' => 10,
            // Any other character is worth 0 points
            _ => 0,
        })
        .sum() // Sum all point values and return the total
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::*; // Bring `score` function into scope for testing

    #[test]
    fn test_simple() {
        // Test lowercase and uppercase single letters
        assert_eq!(score("a"), 1);
        assert_eq!(score("A"), 1);
        assert_eq!(score("h"), 4);
        // Test short words
        assert_eq!(score("at"), 2);
        assert_eq!(score("Yes"), 6);
        assert_eq!(score("cellphones"), 17);
    }

    #[test]
    fn test_empty() {
        // Test empty string and a single space
        assert_eq!(score(""), 0);
        assert_eq!(score(" "), 0);
    }

    #[test]
    fn test_special() {
        // Test sentence with uppercase letters, accents, punctuation, and spaces
        assert_eq!(score("in Portugal NÃO stands for: no"), 30);
        assert_eq!(score("This is a test, comparação, têm Água?"), 36);
    }

    #[test]
    fn test_long() {
        // Test mixed case sentence
        assert_eq!(score("ThiS is A Test"), 14);
        // Test longer sentence
        assert_eq!(score("long sentences are working"), 34);
        // Test all letters of the alphabet
        assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
    }
}
