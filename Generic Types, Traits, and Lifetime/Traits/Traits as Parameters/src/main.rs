use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

// Trait as a parameter
pub fn notify(item: &impl Summary) {
    println!("Notify! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Notify2! {}", item.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Notify3! {}", item1.summarize());
    println!("Notify3! {}", item2.summarize());
}

pub fn notify3_2(item1: &impl Summary, item2: &impl Summary) {
    println!("Notify3.2! {}", item1.summarize());
    println!("Notify3.2! {}", item2.summarize());
}

// Multiple Trait Bounds with the + Syntax
pub fn notify4(item: &(impl Summary + Display)) {
    println!("Notify4! {} {}", item.summarize(), item);
}

pub fn notify5<T: Summary + Display>(item: &T) {
    println!("Notify5! {} {}", item.summarize(), item);
    // println!("Notify5! {}", item.clone()); //item.clone() does nothing
}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("Some function : {}", t);
    println!("{:?}", u);
    println!("{} {:?}", t.clone(), u.clone());
    0
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("Some function2 : {}", t);
    println!("{:?}", u);
    println!("{} {:?}", t.clone(), u.clone());
    0
}

type Int = i32;
impl Summary for Int {
    fn summarize(&self) -> String {
        format!("Int: {}", self)
    }
}

type Float = f64;
impl Summary for Float {
    fn summarize(&self) -> String {
        format!("Float: {}", self)
    }
}

fn main() {
    let int: Int = 3;
    let int2: Int = 4;
    let float: Float = 3.5;
    notify(&int);
    notify2(&int);
    notify3(&int, &int2);
    notify3_2(&int, &float);
    notify4(&float);
    notify5(&float);
    some_function(&int, &int2);
    some_function2(&int2, &float);
}
