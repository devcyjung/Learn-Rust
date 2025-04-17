use crate::List::{Cons, Nil};
// use std::mem;
fn main() {
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    for (i, v) in l.iter().enumerate() {
        println!("index {i}: {:?}", v.value());
    }
    println!("len: {}", l.len());
    println!("head: {:?}", l.head());
    println!("tail: {:?}", l.tail());

    let m = Cons(4, Box::new(Cons(5, Box::new(Nil))));
    // let n = l.concat(&m);
    println!("l: {l:?}");
    println!("m: {m:?}");
    // println!("n: {n:?}");

    for (x, y) in l.iter().chain(m.iter()).enumerate() {
        println!("{x:?},{y:?}");
    }

    let mut x = 5;
    let y = &mut x;
    *y = 6;
    println!("y={y}");
    println!("x={x}");

    let x = 5;
    let mut y = Box::new(x);
    *y = 6;
    println!("y={y}");
    println!("x={x}");

    let mut x = 5;
    let y = Box::new(&mut x);
    **y = 6;
    println!("y={y}");
    println!("x={x}");
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
enum List<T> {
    #[default]
    Nil,
    Cons(T, Box<Self>),
}

struct ListIterator<'a, T>(&'a List<T>);
// struct ListIteratorMut<'a, T>(&'a mut List<T>);

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a List<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Cons(_, next) => {
                let t = self.0;
                self.0 = next;
                Some(t)
            }
            Nil => None,
        }
    }
}

// impl<'a, T> Iterator for ListIteratorMut<'a, T> {
//     type Item = &'a mut List<T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0.is_nil() {
//             return None;
//         }
//         let old_head = mem::replace(&mut self.0, &mut self.0.next_list().unwrap());
//         self.0 = old_head;
//         Some(self.0)
//     }
// }

impl<T> FromIterator<Self> for List<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().next().unwrap_or_else(|| Nil)
    }
}

impl<'a, T> FromIterator<Self> for &'a List<T>
where
    Self: 'a,
{
    fn from_iter<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().next().unwrap_or_else(|| &Nil)
    }
}

impl<T> List<T> {
    const fn new() -> Self {
        Nil
    }

    const fn is_nil(&self) -> bool {
        matches!(self, Nil)
    }

    const fn iter(&self) -> ListIterator<T> {
        ListIterator(self)
    }

    const fn value(&self) -> Option<&T> {
        match self {
            Nil => None,
            Cons(val, _) => Some(val),
        }
    }

    const fn next_list(&self) -> Option<&List<T>> {
        match self {
            Nil => None,
            Cons(_, l) => Some(l),
        }
    }

    // fn concat<'a>(&'a self, other: &'a Self) -> &'a Self {
    //     let last = self.iter().last();
    //     match last {
    //         None => (),
    //         Some(l) => {
    //             *l = Cons();
    //         }
    //     }
    //     self.iter().chain(other.iter()).collect()
    // }

    fn len(&self) -> usize {
        self.iter().count()
    }

    fn head(&self) -> Option<&Self> {
        self.iter().next()
    }

    fn tail(&self) -> Option<&Self> {
        self.iter().last()
    }
}
