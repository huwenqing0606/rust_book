extern crate adder;

mod common;

#[test]
fn it_adds_two() {
	assert_eq!(4, adder::add_two(2));
}

#[test]
fn common_setup() {
	assert_eq!(3, common::setup());
}