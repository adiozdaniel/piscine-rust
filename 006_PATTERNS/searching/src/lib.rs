// Define a public function `search` that performs binary search on a slice of i32 values
// Returns the index of `key` if found, or None if not found
pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let array = array.as_ref(); // Ensure the input is treated as a slice

    // Return None if the array is empty
    if array.is_empty() {
        return None;
    }

    let mut left: usize = 0; // Initialize the left boundary of the search range
    let mut right: usize = array.len(); // Initialize the right boundary (exclusive)

    // Continue searching while the left index is less than or equal to the right index
    while left <= right {
        let middle: usize = (left + right) / 2; // Calculate the middle index

        let element = array.get(middle)?; // Attempt to get the element at the middle index, return None if out of bounds

        // If the key is less than the current element, search the left half
        if key < *element {
            right = middle.checked_sub(1)?; // Safely subtract 1 from middle to prevent underflow
        }
        // If the key is greater than the current element, search the right half
        else if key > *element {
            left = middle + 1; // Move the left index past the middle
        }
        // If the key matches the element, return the index
        else {
            return Some(middle);
        }
    }

    return None; // Return None if the key is not found
}

// Unit tests for the `search` function
#[cfg(test)]
mod tests {
    use super::*; // Bring `search` function into scope

    #[test]
    fn test_search_a_value_in_an_array() {
        // Test single-element and two-element arrays
        assert_eq!(search(&[6], 6), Some(0));
        assert_eq!(search(&[1, 2], 1), Some(0));
        assert_eq!(search(&[1, 2], 2), Some(1));
    }

    #[test]
    fn test_middle_of_an_array() {
        // Test finding a value in the middle of an array
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }

    #[test]
    fn test_beginning_of_an_array() {
        // Test finding the first element in the array
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
    }

    #[test]
    fn test_end_of_an_array() {
        // Test finding the last element in the array
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
    }

    #[test]
    fn test_long_array() {
        // Test finding values in a longer array
        assert_eq!(
            search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
            Some(9)
        );
        assert_eq!(
            search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
            Some(5)
        );
    }

    #[test]
    fn test_value_is_not_included() {
        // Test searching for values not in the array and in an empty array
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 7), None);
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 0), None);
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 13), None);
        assert_eq!(search(&[], 1), None);
    }
}
