use example;

mod common;

#[test]
fn it_really_adds_two() {
    common::setup();
    assert_eq!(5, example::add_two(3));
}