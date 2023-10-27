mod closures;
mod iterators;

pub fn closures() {
    // closures::closure();

    closures::closure_scope();

    closures::closure_move();
}

pub fn iterators() {
    iterators::create();

    iterators::adaptors();
}
