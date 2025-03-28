mod front_of_house {
    mod hosting {
        pub fn pub1() {}
        pub fn pub2() {}
    }

    pub mod serving {
        use super::hosting::pub1;
        pub use super::hosting::pub2;
        pub fn pub3() {
            pub1()
        }
    }
}

use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    // serving::pub1();
    serving::pub2();
    serving::pub3();
}
