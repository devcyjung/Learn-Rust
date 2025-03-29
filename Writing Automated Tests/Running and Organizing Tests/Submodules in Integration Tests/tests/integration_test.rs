// Direct child files in tests directory serve as integration tests. Each file is a crate.
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, submodules::add_two(2));
}
