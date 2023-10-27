mod ownership;
mod reference;
mod slice;

pub fn ownership() {
    ownership::variable_scope();
    ownership::string_type();
    ownership::variable_move();
    ownership::ownership_func();
    ownership::ownership_return_back();
    ownership::ownership_return_multi_values();
}

pub fn references() {
    reference::references();
    reference::mutable_references();
    reference::data_race();
    reference::dangling_references();
}

pub fn slice_type() {
    slice::first_word_func();
    slice::slices();
    slice::first_word_slice();
    slice::array_slices();
}
