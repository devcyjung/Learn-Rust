use std::fmt::Debug;

#[derive(Debug)]
pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
        T: Debug,
    {
        let f_res = f();
        println!("My Option<T> {self:?}.unwrap_or_else(|| {f_res:?})");
        match self {
            Self::None => f_res,
            Self::Some(x) => x,
        }
    }
}

fn main() {
    let t = Option::None;
    let u = Option::Some(1);
    for i in [t, u] {
        let m = vec![1, 2, 3];
        println!(
            "Print {}",
            i.unwrap_or_else(move || {
                let mut m1 = m;
                m1.push(4);
                println!("Closure: {m1:?}");
                m1.clear();
                println!("Closure Ended: {m1:?}");
                0
            })
        );
    }
}
