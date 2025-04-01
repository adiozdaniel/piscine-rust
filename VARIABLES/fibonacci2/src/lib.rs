// the fibonacci series is a series of numbers in which each number is the sum of the two preceding ones, usually starting with 0 and 1.
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// the tenth element in fibonacci series is 0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main() {
        println!(
            "The element in the position {} in fibonacci series is {}",
            2,
            fibonacci(2)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            4,
            fibonacci(4)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            22,
            fibonacci(22)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            20,
            fibonacci(20)
        );
    }
}
