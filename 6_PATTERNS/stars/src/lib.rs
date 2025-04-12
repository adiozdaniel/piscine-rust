// Define a function named `stars` that takes an unsigned 32-bit integer `n` and returns a `String`
pub fn stars(n: u32) -> String {
    let base: u32 = 2; // Define the base value as 2
    let mut f = 0; // Initialize a counter variable `f` to 0
    let mut l = "".to_string(); // Create an empty String `l` to accumulate asterisks
    while f != base.pow(n) { // Loop until `f` equals 2 raised to the power of `n`
        l += "*"; // Append an asterisk to the string `l`
        f += 1 // Increment the counter `f`
    }
    return l; // Return the final string `l`
}

// Define a test module to verify the correctness of the `stars` function
#[cfg(test)]
mod test {
    use super::*; // Import everything from the outer scope

    #[test]
    fn test_stars() {
        // Test case: 2^0 = 1 star
        assert_eq!(stars(0), "*");
        // Test case: 2^1 = 2 stars
        assert_eq!(stars(1), "**");
        // Test case: 2^2 = 4 stars
        assert_eq!(stars(2), "****");
        // Test case: 2^3 = 8 stars
        assert_eq!(stars(3), "********");
        // Test case: 2^4 = 16 stars
        assert_eq!(stars(4), "****************");
        // Test case: 2^5 = 32 stars
        assert_eq!(stars(5), "********************************");
        // Test case: 2^10 = 1024 stars
        assert_eq!(
            stars(10),
            "****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************"
        );
    }
}
