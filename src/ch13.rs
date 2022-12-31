mod closures;
mod iterators;

pub(crate) fn closures() {
    // closures::closure();

    closures::closure_scope();

    closures::closure_move();
}

pub(crate) fn iterators() {
    iterators::create();

    iterators::adaptors();
}
