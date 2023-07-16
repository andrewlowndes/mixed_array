use mixed_array::mixed_array;
use std::cmp::Ordering;

// define a struct with a generic so the generated types are different
#[derive(Clone, Debug)]
struct Item<T> {
    id: u32,
    #[allow(dead_code)]
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

    dbg!(items.iter().min(), items.iter().max());
}
