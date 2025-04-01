# find_factorial

## Succeeded

950 B

### Instructions

Create a function named `factorial` which returns the factorial of a given number.

The factorial of a number is the product of all the integers from 1 to that number. The special cases are:

- The factorial of 0 is 1.
- The factorial of 1 is 1.

### Function

```rust
pub fn factorial(num: u64) -> u64 {
    // Return the factorial of num
}
```

### Example

As a reminder, the factorial of 6 (written 6!) is calculated as:

```
1 * 2 * 3 * 4 * 5 * 6 = 720
```

### Usage

Here is a possible program to test your function:

```rust
use find_factorial::*;

fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}
```

### Output

```bash
$ cargo run
The factorial of 0 = 1
The factorial of 1 = 1
The factorial of 5 = 120
The factorial of 10 = 3628800
The factorial of 19 = 121645100408832000
$
```

---
