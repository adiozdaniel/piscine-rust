// Levenshtein's distance using two matrix rows

use std::mem;

// This function calculates the Levenshtein edit distance between two strings `source` and `target`.
// It uses an optimized approach by maintaining only two rows of the distance matrix at a time.
pub fn edit_distance(source: &str, target: &str) -> usize {
    // Get the lengths of the source and target strings
    let m = source.len();
    let n = target.len();

    // Initialize two vectors (v0 and v1) to represent the current and previous rows of the matrix
    // v0 will hold the previous row values, v1 will hold the current row values
    let mut v0 = (0..=n).collect::<Vec<_>>();
    let mut v1 = vec![0; n + 1];

    // Iterate through each character in the source string
    for i in 0..m {
        // Set the first element of v1 to be the cost of deleting all characters up to index `i`
        v1[0] = i + 1;

        // Iterate through each character in the target string
        for j in 0..n {
            // Calculate the cost of deleting a character from the source string
            let deletion_cost = v0[j + 1] + 1;
            // Calculate the cost of inserting a character into the source string
            let insertion_cost = v1[j] + 1;
            // Calculate the cost of substituting a character in the source string
            let substitution_cost = v0[j]
                + if source.chars().nth(i) == target.chars().nth(j) {
                    0
                } else {
                    1
                };

            // Set v1[j + 1] to the minimum of the three possible costs (deletion, insertion, substitution)
            v1[j + 1] = [deletion_cost, insertion_cost, substitution_cost]
                .into_iter()
                .min()
                .unwrap();
        }

        // Swap v0 and v1 for the next iteration, so that v1 becomes the previous row for the next source character
        mem::swap(&mut v0, &mut v1);
    }

    // The final value in v0 is the Levenshtein distance between the source and target strings
    v0[n]
}
