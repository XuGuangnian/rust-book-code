pub mod string;
pub mod vector;
pub mod hashmap;
pub mod exercise;

pub fn exercises() {
    exercise::one();

    exercise::two();

    exercise::three();
}

pub(crate) fn hashmap() {
    hashmap::create();

    hashmap::ownership();

    hashmap::access_values();

    hashmap::iterate();

    hashmap::update();
}

pub(crate) fn string() {
    string::create();

    string::update();

    string::combine_two_strings();

    string::combine_multiple_strings();

    string::not_indexed();

    string::slice();

    string::iterate();
}

pub(crate) fn vector() {
    vector::create();

    vector::update();

    vector::drop();

    vector::read_element();

    vector::vector_work_type();

    vector::iterate();

    vector::iterate_mut();

    vector::store_multiple_types_by_enum();
}