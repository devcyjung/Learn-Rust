#![feature(type_alias_impl_trait)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

trait BiComparable {
    type PrimaryOrder;
    type SecondaryOrder;
    fn primary(&self) -> Self::PrimaryOrder;
    fn secondary(&self) -> Self::SecondaryOrder;
}
impl BiComparable for Rectangle {
    type PrimaryOrder = u32;
    type SecondaryOrder = u32;

    fn primary(&self) -> u32 {
        self.width
    }

    fn secondary(&self) -> u32 {
        self.height
    }
}

fn main() {
    fn bi_key_extractor<S>() -> impl FnMut(&S) -> (S::PrimaryOrder, S::SecondaryOrder)
    where
        S: BiComparable,
    {
        |r| (r.primary(), r.secondary())
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 15,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 3,
            height: 6,
        },
        Rectangle {
            width: 8,
            height: 1,
        },
    ];

    // Using `sort_by_key` and a closure to sort a list
    // of `Rectangle` instances by their `width` value
    println!("Pre sort: {list:?}");
    list.sort_by_key(bi_key_extractor());
    println!("Post sort: {list:?}");

    // Attempting to use an FnOnce closure with sort_by_key
    let mut ref_vec: Vec<&Rectangle> = vec![];
    // let mut val_vec: Vec<Rectangle> = vec![];
    let value = Rectangle {
        width: 0,
        height: 0,
    };

    list.sort_by_key(|r| {
        // val_vec.push(value);
        ref_vec.push(&value);
        println!("{ref_vec:?}");
        r.width
    });
    println!(
        "Post sort by width: {list:?}, sorted in {} operations",
        ref_vec.len()
    );

    // Using an FnMut closure with sort_by_key is allowed
    let mut num_sort_operations = 0;

    // Why is sort_by_key designed to accept FnMut but not FnOnce?
    // It needs to call this closure more than once. That's why
    // But it doesn't need to mutate anything. It can if you need.
    // FnOnce moves something in the enclosing scope to somewhere else.
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        (-i64::from(r.width), -i64::from(r.height))
    });

    println!("{list:?}, sorted in {num_sort_operations} operations");
}
