pub mod exercise;
pub mod hashmap;
pub mod string;
pub mod vector;

pub fn exercises() {
    exercise::one();
    exercise::two();
    exercise::three();
}

pub fn hashmap() {
    hashmap::create();
    hashmap::ownership();
    hashmap::access_values();
    hashmap::iterate();
    hashmap::update();
}

pub fn string() {
    string::create();
    string::update();
    string::combine_two_strings();
    string::combine_multiple_strings();
    string::not_indexed();
    string::slice();
    string::iterate();
}

pub fn vector() {
    vector::create();
    vector::update();
    vector::drop();
    vector::read_element();
    vector::vector_work_type();
    vector::iterate();
    vector::iterate_mut();
    vector::store_multiple_types_by_enum();
}
