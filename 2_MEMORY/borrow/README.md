# str_len Function

## Description

The `str_len` function calculates and returns the length of a given string slice (`&str`).

## Function Signature

```rust
pub fn str_len(s: &str) -> usize {
    s.len()
}
```

## Usage

This function takes a string slice as input and returns its length as a `usize` value.

### Example

```rust
use borrow::*;

fn main() {
    let s = "hello";
    let s1 = "camelCase".to_string();
    let s2 = "olá!";

    println!("\tstr_len(\"{}\") = {}", s, str_len(s));
    println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
    println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}
```

#### Output

```
str_len("hello") = 5
str_len("camelCase") = 9
str_len("olá!") = 4
```

## Tests

The following test cases ensure that the function works correctly for different types of input:

```rust
#[cfg(test)]
mod tests {
    use super::str_len;

    #[test]
    fn empty_string() {
        assert_eq!(str_len(""), 0);
    }

    #[test]
    fn ascii_string() {
        assert_eq!(str_len("hello"), 5);
    }

    #[test]
    fn string_with_spaces() {
        assert_eq!(str_len("hello world"), 11);
    }

    #[test]
    fn string_with_special_chars() {
        assert_eq!(str_len("!@#$%^&*()"), 10);
    }

    #[test]
    fn string_with_mixed_chars() {
        assert_eq!(str_len("Rust 🦀 is cool!"), 15); // Note: crab emoji is 4 bytes
    }
}
```

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Notions

- [Primitive Type str](https://doc.rust-lang.org/std/primitive.str.html)
