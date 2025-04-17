use crate::List::{Cons, Nil};

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[must_use]
pub const fn create_empty_list() -> List {
    Nil
}

#[must_use]
pub fn create_non_empty_list() -> List {
    Cons(0, Box::new(Cons(1, Box::new(Nil))))
}
