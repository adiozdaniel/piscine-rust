// Define a public function that converts a number to its ordinal string representation
pub fn num_to_ordinal(x: u32) -> String {
    format!(
        "{}{}", // Format the number followed by its ordinal suffix
        x,
        match (x % 10, x % 100) {
            // Match on the last digit and the last two digits of the number
            (1, 1) | (1, 21..=91) => "st", // Ends with 1 (excluding 11, 111, etc.)
            (2, 2) | (2, 22..=92) => "nd", // Ends with 2 (excluding 12, 112, etc.)
            (3, 3) | (3, 23..=93) => "rd", // Ends with 3 (excluding 13, 113, etc.)
            _ => "th", // All other cases get "th" suffix
        }
    )
}

// Unit tests for the `num_to_ordinal` function
#[test]
fn test_num_to_ordinal() {
    // Test for 0, which should return "0th"
    assert_eq!(num_to_ordinal(0), "0th");
    // Test common ordinal suffixes
    assert_eq!(num_to_ordinal(1), "1st");     // ends with 1
    assert_eq!(num_to_ordinal(12), "12th");   // ends with 12, special case
    assert_eq!(num_to_ordinal(22), "22nd");   // ends with 2
    assert_eq!(num_to_ordinal(43), "43rd");   // ends with 3
    assert_eq!(num_to_ordinal(67), "67th");   // general case
    assert_eq!(num_to_ordinal(1901), "1901st"); // large number, ends with 1
}
