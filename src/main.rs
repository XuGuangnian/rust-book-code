#![allow(unused)]

mod ch8;

use crate::ch8::string;
use crate::ch8::vector;
use crate::ch8::hashmap;
use crate::ch8::exercise;

fn main() {
    // ch8_vector();

    // ch8_string();

    // ch8_hashmap();

    ch8_exercises();
}

fn ch8_exercises() {
    exercise::one();

    exercise::two();

    exercise::three();
}

fn ch8_hashmap() {
    hashmap::create();

    hashmap::ownership();

    hashmap::access_values();

    hashmap::iterate();

    hashmap::update();
}

fn ch8_string() {
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
