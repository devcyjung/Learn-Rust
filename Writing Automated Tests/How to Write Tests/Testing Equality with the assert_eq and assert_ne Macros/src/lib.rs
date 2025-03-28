pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod adder_tests {

    #[test]
    fn it_adds_two() {
        assert_eq!(4, super::add_two(2));
        assert_ne!(0, super::add_two(1));
        assert_eq!(10, super::add_two(8));
    }

    #[test]
    fn it_adds_two_v2() {
        assert_eq!(5, super::add_two(3));
    }
}
