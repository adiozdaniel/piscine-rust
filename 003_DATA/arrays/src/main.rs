use arrays::*;

// TODO: Correct the main.rs
fn main() {
    let a = (1..=10)._;
    let b = [_];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}
