mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }
}

// do not use self
use self::front_of_house::hosting as hosting_alias;

use crate::front_of_house::hosting;

// don't use individual function. (enum, struct, other items are ok)
use crate::front_of_house::hosting::seat_at_table;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting_alias::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    seat_at_table()
}

use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

use std::fmt;
use std::io;

fn collision() {
    let fmt_result1: FmtResult = Ok(());
    let io_result1: IoResult<()> = Ok(());
    let fmt_result2: fmt::Result = Ok(());
    let io_result2: io::Result<()> = Ok(());
}
