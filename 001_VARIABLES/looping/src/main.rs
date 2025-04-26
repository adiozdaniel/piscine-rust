// Title: Looping
use std::io;

fn main() {
    let mut tries = 0;
    let mut answer = String::new();
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim() == "The letter e" {
            tries += 1;
            println!(
                "Number of trials: {}",
                tries
            );
            break;
        } else {
            tries += 1;
            // println!("Try again!");
            answer.clear();
        }
    }
}
