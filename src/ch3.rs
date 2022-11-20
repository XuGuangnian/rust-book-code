pub mod data_type;
pub mod variables;

pub(crate) fn variables() {
    variables::immut_variables();
    variables::mut_variables();
    variables::constants();
    variables::shadowing();
}

pub(crate) fn data_type() {
    data_type::integer();
    data_type::float();
    data_type::calc();
    data_type::bool();
    data_type::char();
    data_type::tuple();
    data_type::array();
}
