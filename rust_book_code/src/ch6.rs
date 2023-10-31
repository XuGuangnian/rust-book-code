mod enums;
mod if_let;
mod matches;

pub fn enumerate() {
    enums::enum_definition();
    enums::enum_and_struct();
    enums::enum_option();
}

pub fn match_control_flow() {
    matches::match_coin();
    matches::bind_values();
    matches::match_option();
    matches::match_catch_all();
}

pub fn if_let() {
    if_let::match_one_pattern();
    if_let::if_let();
    if_let::coin();
}
