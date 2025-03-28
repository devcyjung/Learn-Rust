mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/***
* src
* - front_of_house
* - · hosting.rs: pub fn add_to_waitlist() {}
* - front_of_house.rs: pub mod hosting;
* - lib.rs: this file
*/
