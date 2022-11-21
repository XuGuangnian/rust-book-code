mod enums;
mod if_let;
mod matches;

pub(crate) fn enumerate() {
    enums::enum_definition();
    enums::enum_and_struct();
    enums::enum_option();
}

pub(crate) fn match_control_flow() {
    matches::match_coin();
    matches::bind_values();
    matches::match_option();
    matches::match_catch_all();
}

pub(crate) fn if_let() {
    if_let::match_one_pattern();
    if_let::if_let();
    if_let::coin();
}
