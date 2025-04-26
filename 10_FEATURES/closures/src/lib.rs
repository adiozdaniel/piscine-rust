pub fn first_fifty_even_square() -> Vec<i32> {
    (1..).map(|x| x * 2)      // Generates even numbers: 2, 4, 6, ...
        .take(50)             // Takes the first 50 even numbers
        .map(|x| x * x)       // Squares each number
        .collect()            // Collects into Vec<i32>
}
