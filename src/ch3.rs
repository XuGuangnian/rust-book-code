pub mod data_type;
pub mod func;
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

pub(crate) fn func() {
    func::func_example();
    func::func_param(5);
    func::print_labeled_measurement(5, 'h');
    func::statement();
    func::expression();
    func::return_value();
}

pub(crate) fn comments() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
