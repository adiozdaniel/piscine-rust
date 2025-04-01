# groceries

## Succeeded

800 B

### Instructions

Create a function named `insert` that inserts a new element at the end of the `Vec`.

Create another function named `at_index` that returns the value found at the index passed as an argument.

### Functions

```rust
pub fn insert(vec: &mut Vec<String>, val: String) {
    // Insert the value at the end of the vector
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    // Return the value found at the specified index
}
```

### Usage

Here is a possible program to test your function:

```rust
use groceries::*;

fn main() {
    let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];
    insert(&mut groceries, String::from("nuts"));
    println!("groceries = {:?}", &groceries);
    println!("groceries[1] = {:?}", at_index(&groceries, 1));
}
```

### Output

```bash
$ cargo run
groceries = ["yogurt", "panettone", "bread", "cheese", "nuts"]
groceries[1] = "panettone"
$
```

### Notions

- [Common Collections](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html)
- [Vectors](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)

---
