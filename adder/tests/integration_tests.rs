mod common;

use adder::add_two;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
