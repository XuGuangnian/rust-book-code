use rust_book_code;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_book_code::add_two(2));
}
