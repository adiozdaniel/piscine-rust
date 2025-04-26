// Import the HashSet collection from the standard library
use std::collections::HashSet;

// Define a public function that checks if a given string is a pangram
pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new(); // Create a new HashSet to store unique letters

    // Iterate over each character in the string, converting to lowercase
    for c in s.chars().map(|c| c.to_ascii_lowercase()) {
        match c {
            'a'..='z' => letters.insert(c), // Insert letters a-z into the set
            _ => false, // Ignore non-alphabetic characters
        };
        if letters.len() == 26 {
            return true; // Return true early if all 26 letters are found
        }
    }

    false // Return false if not all letters were found
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_empty_strings() {
        assert!(!is_pangram("")); // Empty string is not a pangram
        assert!(!is_pangram(" ")); // Space is not a pangram
    }

    #[test]
    fn test_is_panagram() {
        // Valid pangrams containing all letters
        assert!(is_pangram("the quick brown fox jumps over the lazy dog"));
        assert!(is_pangram("the_quick_brown_fox_jumps_over_the_lazy_dog"));
        assert!(is_pangram("the 1 quick brown fox jumps over the 2 lazy dogs"));
    }

    #[test]
    fn test_not_pangrams() {
        // Test strings that are missing some letters
        assert!(!is_pangram("a quick movement of the enemy will jeopardize five gunboats"));
        assert!(!is_pangram("the quick brown fish jumps over the lazy dog"));
        assert!(!is_pangram("the quick brown fox jumps over the lay dog"));
        assert!(!is_pangram("7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog"));
        // Test cases that are pangrams with special characters
        assert!(is_pangram("\"Five quacking Zephyrs jolt my wax bed.\""));
        assert!(is_pangram("Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich."));
    }
}
