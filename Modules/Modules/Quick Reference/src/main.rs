use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

/***
* src
* - garden
* - · vegetables.rs     ->  pub struct Asparagus {}
* · garden.rs           ->  pub mod vegetables;
* · main.rs
*/
