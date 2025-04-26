// Define a public function `number_logic` that checks if a number equals the sum
// of its digits each raised to the power of the number of digits (Armstrong number check)
pub fn number_logic(num: u32) -> bool {
    let s = num.to_string(); // Convert the number to a string
    let len = s.len(); // Get the length of the string (number of digits)

    // Compare the original number with the sum of each digit raised to the power of `len`
    num == s
        .chars() // Iterate over each character in the string
        .map(|x| x.to_digit(10).unwrap().pow(len as u32)) // Convert to digit and raise to power
        .sum() // Sum all the powered digits
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_zero() {
        // Test case for 0 which is considered a valid Armstrong number
        assert!(number_logic(0))
    }

    #[test]
    fn test_single_digit_numbers() {
        // All single-digit numbers are Armstrong numbers (e.g., 5^1 = 5)
        assert!(number_logic(5));
        assert!(number_logic(9))
    }

    #[test]
    fn test_two_digit_numbers() {
        // Most two-digit numbers are not Armstrong numbers
        assert!(!number_logic(10))
    }

    #[test]
    fn test_three_or_more_digit_number() {
        // Valid Armstrong numbers
        assert!(number_logic(153)); // 1^3 + 5^3 + 3^3 = 153
        assert!(!number_logic(100)); // 1^3 + 0^3 + 0^3 ≠ 100
        assert!(number_logic(9474)); // 9^4 + 4^4 + 7^4 + 4^4 = 9474
        assert!(!number_logic(9475)); // Fails the Armstrong condition
        assert!(number_logic(9_926_315)); // Valid large Armstrong number
        assert!(!number_logic(9_926_316)) // Fails by 1
    }
}
