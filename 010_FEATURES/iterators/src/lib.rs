// Derive Copy and Clone traits to allow Collatz struct to be copied
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,  // Current value in the Collatz sequence
}

impl Collatz {
    // Constructor for Collatz that takes an initial value
    pub fn new(aux: u64) -> Self {
        Collatz { v: aux }  // Create new Collatz instance with given value
    }
}

// Implement Iterator trait for Collatz to generate sequence
impl Iterator for Collatz {
    type Item = Collatz;  // Each item in the iterator is a Collatz struct

    // The next() method generates the next value in the sequence
    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            None  // Sequence ends when value reaches 1
        } else {
            let old_value = self.v;  // Store current value before modifying
            if self.v % 2 == 0 {
                self.v /= 2;  // If even, divide by 2
            } else {
                self.v = self.v * 3 + 1;  // If odd, multiply by 3 and add 1
            }
            Some(Collatz { v: old_value })  // Return previous value
        }
    }
}

// Function to calculate Collatz sequence length for a given number
pub fn collatz(n: u64) -> usize {
    let nb = Collatz::new(n);  // Create Collatz iterator
    nb.count()  // Count number of elements in the sequence
}
