# fibonacci2

## Succeeded

950 B

### Instructions

Complete the body of the function `fibonacci`.

The function `fibonacci(n)` receives a number `n` and returns the nth number in the Fibonacci series.

### Fibonacci Series

The Fibonacci series starts like this:

```
0, 1, 1, 2, 3, 5, 8, 13, ...
```

The Fibonacci sequence is defined as:

- F(0) = 0
- F(1) = 1
- F(n) = F(n-1) + F(n-2) for n > 1

### Function

```rust
pub fn fibonacci(n: u32) -> u32 {
    // Complete the function to return the nth number in the Fibonacci series
}
```

### Usage

Here is a possible test for your function:

```rust
use fibonacci2::*;

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
```

### Output

```bash
$ cargo run
The element in the position 2 in fibonacci series is 1
The element in the position 4 in fibonacci series is 3
The element in the position 22 in fibonacci series is 17711
The element in the position 20 in fibonacci series is 6765
$
```

---

## Notions

- [Primitives](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
- [Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)
