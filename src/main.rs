#![allow(unused)]

mod ch8;

use crate::ch8::string;
use crate::ch8::vector;

fn main() {
    // ch8_vector();

    string::create();

    string::update();

    string::combine_two_strings();

    string::combine_multiple_strings();

    string::not_indexed();

    string::slice();

    string::iterate();

}

fn ch8_vector() {
    vector::create();

    vector::update();

    vector::drop();

    vector::read_element();

    vector::vector_work_type();

    vector::iterate();

    vector::iterate_mut();

    vector::store_multiple_types_by_enum();
}
