//each test file = crate
extern crate rust_program; //add library crate
use rust_program::rust_book::tests;
mod common; //using common for all tests functions
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
