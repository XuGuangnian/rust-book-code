pub(crate) fn panic_demo() {
    panic!("crash and burn");
}

pub(crate) fn index_out_bounds() {
    let v = vec![1, 2, 3];

    v[99];
}
