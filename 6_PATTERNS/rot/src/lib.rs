// Define a public function `rotate` which performs a Caesar cipher shift on the input string
pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars() // Iterate through each character in the input string
        .map(|character| match character {
            // If character is a lowercase letter
            'a'..='z' => {
                // Shift within the 'a' to 'z' range using modular arithmetic
                ((((character as u8) - b'a') as i8 + key).rem_euclid(26) as u8 + b'a') as char
            }
            // If character is an uppercase letter
            'A'..='Z' => {
                // Shift within the 'A' to 'Z' range using modular arithmetic
                ((((character as u8) - b'A') as i8 + key).rem_euclid(26) as u8 + b'A') as char
            }
            // Leave all other characters unchanged
            _ => character,
        })
        .collect::<String>() // Collect the transformed characters into a new String
}

#[cfg(test)]
mod test {
    use super::*; // Bring all items from the outer scope into the test module

    #[test]
    fn test_simple() {
        // Simple rotation tests with lowercase and uppercase letters
        assert_eq!("z", rotate("m", 13));
        assert_eq!("n", rotate("m", 1));
        assert_eq!("a", rotate("a", 26)); // Full cycle returns original
        assert_eq!("z", rotate("a", 25));
        assert_eq!("b", rotate("l", 16));
        assert_eq!("j", rotate("j", 0)); // Zero rotation
        assert_eq!("RNXX", rotate("MISS", 5)); // Uppercase shift
        assert_eq!("M J Q Q T", rotate("H E L L O", 5)); // Mixed case with space
    }

    #[test]
    fn test_all_letters() {
        // Full sentence with rotation by 13 (ROT13)
        assert_eq!(
            "Gur svir obkvat jvmneqf whzc dhvpxyl.",
            rotate("The five boxing wizards jump quickly.", 13)
        );
    }

    #[test]
    fn test_numbers_punctuation() {
        // Rotation should ignore digits and punctuation
        assert_eq!(
            "Xiwxmrk amxl ryqfivw 1 2 3",
            rotate("Testing with numbers 1 2 3", 4)
        );
        assert_eq!("Gzo\'n zvo, Bmviyhv!", rotate("Let\'s eat, Grandma!", 21));
    }

    #[test]
    fn test_negative() {
        // Test rotation with negative keys (left shift)
        assert_eq!("z", rotate("a", -1));
        assert_eq!("W", rotate("A", -4));
        assert_eq!("Fqefuzs", rotate("Testing", -14));
    }
}
