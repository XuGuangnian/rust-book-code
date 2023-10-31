mod closures;
mod iterators;

pub fn closures() {
    // new book code
    closures::closure_demo();
    closures::closure_var();
    closures::closure_ownership();
    closures::closure_fn();

    // old examples
    closures::closure();
    closures::closure_scope();
    closures::closure_move();
}

pub fn iterators() {
    iterators::create();
    iterators::adaptors();
}
