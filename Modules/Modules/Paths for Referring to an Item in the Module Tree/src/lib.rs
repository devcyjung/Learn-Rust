mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist");
            seat_at_table();
        }

        fn seat_at_table() {
            println!("Seat at table");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path: ERROR!!! access to private components
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: ERROR!!! access to private components
    front_of_house::hosting::add_to_waitlist();
}
