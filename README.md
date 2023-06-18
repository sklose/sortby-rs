[![CI](https://github.com/sklose/sortby-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/sklose/sortby-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/sortby.svg)](https://crates.io/crates/sortby)
[![docs.rs](https://img.shields.io/docsrs/sortby)](https://docs.rs/sortby)

# Sort By

Convenience functions that allow for sorting iterators.

# Example

```rust
use sortby::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Person {
  pub age: i32,
  pub name: &'static str,
}

fn main() {
  let data = vec![
    Person {
      name: "Rich",
      age: 18,
    },
    Person {
      name: "Bob",
      age: 9,
    },
    Person {
      name: "Marc",
      age: 21,
    },
    Person {
      name: "Alice",
      age: 18,
    },
  ];

  let sorted: Vec<_> = data.iter()
    .sort_by_desc(|p| p.age)
    .then_sort_by(|p| p.name)
    .collect();

   println!("{:#?}", sorted);
}
```
