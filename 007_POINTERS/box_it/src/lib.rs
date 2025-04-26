// Converts a string ending with 'k' (e.g., "5.5k") into an integer by removing the 'k',
// parsing the float, multiplying by 1000, and returning as u32.
fn convert(s: &str) -> u32 {
    let mut a = s.to_string(); // Create a mutable copy of the input string
    a.pop(); // Remove the 'k' character
    let n = a.parse::<f32>().unwrap(); // Parse the remaining string as a float
    (n * 1000.0) as u32 // Convert to u32 by multiplying by 1000
}

// Transforms a space-separated string of numbers (some ending in 'k') into a Vec<u32>,
// storing it on the heap inside a Box.
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = Vec::new(); // Initialize empty vector

    for token in s.split_whitespace() { // Iterate over each token separated by space
        if token.contains("k") { // If token ends in 'k', convert using custom logic
            v.push(convert(token));
        } else {
            v.push(token.parse::<u32>().unwrap()); // Otherwise, parse as u32 directly
        }
    }
    Box::new(v) // Return vector boxed on the heap
}

// Takes ownership of a Box<Vec<u32>> and returns the inner Vec<u32>
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a // Dereference and return the vector
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // Tests the transformation of input strings to boxed vectors
    #[test]
    fn test_transform() {
        let new_str = String::from("5.5k 8.9k 32"); // Mixed 'k' and normal
        let new_str_1 = String::from("6.8k 13.5k"); // All 'k' values
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992"); // Mixed with multiple elements

        let a = transform_and_save_on_heap(new_str); // Boxed transformation
        let b = transform_and_save_on_heap(new_str_1);
        let c = transform_and_save_on_heap(new_str_2);

        assert_eq!(a, Box::new(vec![5500, 8900, 32])); // Check converted values
        assert_eq!(b, Box::new(vec![6800, 13500]));
        assert_eq!(c, Box::new(vec![20300, 3800, 7700, 992]));
        assert_eq!(mem::size_of_val(&a), 8); // Check size of Box (pointer size)
        assert_eq!(mem::size_of_val(&b), 8);
        assert_eq!(mem::size_of_val(&c), 8);
    }

    // Tests taking ownership of boxed vector and verifying content and memory size
    #[test]
    fn test_take_value_from_box() {
        let new_str = String::from("5.5k 8.9k 32"); // Input for transformation
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");
        let a = take_value_ownership(transform_and_save_on_heap(new_str)); // Transfer ownership
        let b = take_value_ownership(transform_and_save_on_heap(new_str_1));
        let c = take_value_ownership(transform_and_save_on_heap(new_str_2));

        assert_eq!(a, vec![5500, 8900, 32]); // Validate extracted vectors
        assert_eq!(b, vec![6800, 13500]);
        assert_eq!(c, vec![20300, 3800, 7700, 992]);
        assert_eq!(mem::size_of_val(&a), 24); // Size of Vec (3 * u32 = 12 + 12 metadata)
        assert_eq!(mem::size_of_val(&b), 24);
        assert_eq!(mem::size_of_val(&c), 24);
    }
}
