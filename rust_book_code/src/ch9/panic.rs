pub fn panic_demo() {
    panic!("crash and burn");
}

pub fn index_out_bounds() {
    let v = vec![1, 2, 3];

    // v[99];
    assert!(v.len() > 99);
}
