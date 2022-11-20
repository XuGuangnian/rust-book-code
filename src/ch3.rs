mod branches;
mod data_type;
mod exercises;
mod func;
mod variables;

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

pub(crate) fn branches() {
    branches::if_else();
    branches::loop_func();
    branches::loop_label();
    branches::while_func();
    branches::for_element();
}

pub(crate) fn exercises() {
    exercises::temperature_convert();
    exercises::fibonacci();
    exercises::print_lyrics();
}
