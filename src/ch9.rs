mod panic;
mod result;

pub(crate) fn panic() {
    // panic::panic_demo();

    // panic::index_out_bounds();
}

pub(crate) fn result() {
    // result::open_file();

    // result::recoverable_errors();

    // result::unwrap_or_else();
    // result::unwrap_and_expect();

    result::propagate_errors();
    result::propagate_errors_shortcut();
    result::shortcut_chains();
    result::read_to_string();

    result::last_char()
}
