// Define a struct `Numbers` with a lifetime parameter `'a`
// This struct holds a reference to a slice of u32 numbers
#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],  // Reference to a slice of u32 numbers
}

// Implement methods for the `Numbers` struct
impl<'a> Numbers<'a> {
    // Constructor method that creates a new `Numbers` instance
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }  // Return new Numbers instance with the given slice
    }

    // Method that returns a reference to the stored numbers slice
    pub fn list(&self) -> &[u32] {
        self.numbers  // Return the reference to the numbers slice
    }

    // Method that returns the last number in the slice (if any)
    pub fn latest(&self) -> Option<u32> {
        self.numbers.iter().last().map(|u| *u)  // Get last element and dereference it
    }

    // Method that returns the highest number in the slice (if any)
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().map(|u| *u)  // Get maximum element and dereference it
    }

    // Method that returns a vector of the three highest numbers
    pub fn highest_three(&self) -> Vec<u32> {
        // Create a mutable owned copy of the numbers slice
        let mut ordered_numbers = self.numbers.to_owned();
        // Sort the numbers in descending order
        ordered_numbers.sort_by(|a, b| b.cmp(a));
        // Take the first three elements, dereference them, and collect into a Vec
        ordered_numbers.iter().take(3).map(|u| *u).collect()
    }
}
