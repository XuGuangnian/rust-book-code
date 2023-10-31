mod example_structs;
mod method_syntax;
mod structure;

pub fn structure() {
    structure::struct_definition();
    structure::tuple_structs();
    structure::unit_like_struct();
    structure::struct_reference_no_lifetime();
}

pub fn example_structs() {
    example_structs::rectangles();
    example_structs::rectangles_with_tuple();
    example_structs::rectangles_with_struct();
    example_structs::print_struct();
    example_structs::dbg_macro();
}

pub fn method_syntax() {
    method_syntax::impl_area();
    method_syntax::reference_and_dereference();
    method_syntax::can_hold();
    method_syntax::associated_functions();
}
