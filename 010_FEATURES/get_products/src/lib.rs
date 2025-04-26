pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    // Initialize a counter to keep track of the current index being processed
    let mut counter: usize = 0;
    // Initialize a vector to store the resulting products
    let mut product_results: Vec<usize> = Vec::new();

    // If the input array has less than 2 elements, return an empty vector
    // because there are no other elements to multiply
    if arr.len() < 2 {
        return product_results;
    };

    // Iterate over each element in the input array
    for _ in arr.iter() {
        // Initialize the product for the current element
        let mut prod: usize = 1;
        // Create a copy of the input array to work with
        let mut others: Vec<usize> = arr.clone();
        // Remove the current element (at position 'counter') from the copied array
        others.remove(counter);
        // Multiply all remaining elements in the copied array
        for x in others.iter() {
            prod *= *x;
        }
        // Add the computed product to the results vector
        product_results.push(prod);
        // Increment the counter to move to the next element in the next iteration
        counter += 1;
    }
    // Return the vector containing the products
    return product_results;
}
