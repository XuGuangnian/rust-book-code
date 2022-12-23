mod generics;
mod lifetimes;
mod traits;

pub(crate) fn generics() {
    generics::extract_functions();

    generics::generics_function();

    generics::generics_struct();

    generics::generics_enum();
}

pub(crate) fn traits() {
    traits::aggregator();

    traits::trait_parameter();

    traits::trait_return();

    traits::trait_conditional();
}

pub(crate) fn lifetimes() {
    lifetimes::lifetimes_syntax();

    lifetimes::lifetimes_struct();

    lifetimes::lifetimes_static();

    lifetimes::combination();
}
