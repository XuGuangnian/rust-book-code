#![allow(unused_variables)]

mod ch8;

use crate::ch8::vector;

fn main() {
    vector::create();

    vector::update();

    vector::drop();

    vector::read_element();

    vector::vector_work_type();

    vector::iterate();

    vector::iterate_mut();

    vector::store_multiple_types_by_enum();
}
