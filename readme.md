# Mixed Array
Construct arrays of mixed types. Generic enums are created for up to 12 unique types. A macro to generate an array using the correct size macro is provided.

## Usage
1. Include in the `Cargo.toml` file:
```toml
[dependencies]
mixed_array = "0.1.0"
```

2. Import and use the `mixed_array` macro to create an array of items
```rust
use mixed_array::mixed_array;
use std::cmp::Ordering;

// define a struct with a generic so the generated types are different as a demo
#[derive(Clone, Debug)]
struct Item<T> {
    id: u32,
    value: T,
}

impl<T> Eq for Item<T> {}

//implement ordering for our item for any T
impl<T, B> PartialEq<Item<B>> for Item<T> {
    fn eq(&self, other: &Item<B>) -> bool {
        self.id == other.id
    }
}

impl<T, B> PartialOrd<Item<B>> for Item<T> {
    fn partial_cmp(&self, other: &Item<B>) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    //can now create an array of different types
    let items = mixed_array![
        Item::<&str> {
            id: 100,
            value: "blah",
        },
        Item::<u32> {
            id: 200,
            value: 123,
        },
        Item::<f32> {
            id: 300,
            value: 9.3,
        },
    ];

    dbg!(items.into_iter().min());
}
```

## Alternatives
- [**Frunk - HList**](https://crates.io/crates/frunk) - Provides a `hlist`` macro to construct a recursive custom data type used for storing different types (replaces using an array), easy conversion to tuples.
