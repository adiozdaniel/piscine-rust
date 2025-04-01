# Scalar

## Succeeded

500 B

## Description

The `scalar` project involves implementing five arithmetic functions that operate on specific numerical ranges. These functions handle addition, subtraction, multiplication, division, and remainder calculations while ensuring type safety and handling potential overflows.

## Functions

Implement the following functions with appropriate data types:

- **`sum(a, b)`**: Returns the sum of two values from `0` to `255`.
- **`diff(a, b)`**: Returns the difference between two values from `-32768` to `32767`.
- **`pro(a, b)`**: Returns the product of two values from `-128` to `127`.
- **`quo(a, b)`**: Returns the quotient of the division between two 32-bit floating-point numbers.
- **`rem(a, b)`**: Returns the remainder of the division between two 32-bit floating-point numbers.

You must determine the exact data types for the parameters and return values based on the given constraints.

## Example Usage

```rust
use scalar::*;

fn main() {
    // sum
    println!("sum: {}", sum(234, 2)); // 'sum: 236'
    println!("sum: {}", sum(1, 255)); // 'ERROR: attempt to add with overflow'

    // diff
    println!("diff: {}", diff(234, 2)); // 'diff: 232'
    println!("diff: {}", diff(-32768, 32766)); // 'ERROR: attempt to subtract with overflow'

    // product
    println!("pro: {}", pro(23, 2)); // 'pro: 46'
    println!("pro: {}", pro(-128, 2)); // 'ERROR: attempt to multiply with overflow'

    // quotient
    println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'
    println!("quo: {}", quo(-128.23, 2.0)); // 'quo: -64.115'

    // remainder
    println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
    println!("rem: {}", rem(-128.23, 2.0)); // 'rem: -0.22999573'
}
```

## Files to Submit

- `scalar/src/lib.rs`

## Allowed Functions

- Only standard Rust arithmetic operations and built-in error handling mechanisms.

## Notions Covered

- [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
