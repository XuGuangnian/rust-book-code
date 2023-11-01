mod unsafe_rust;

pub fn unsafe_rust() {
    unsafe_rust::create_raw_pointers();
    unsafe_rust::call_unsafe_fn();
    unsafe_rust::create_safe_abstraction();
    unsafe_rust::extern_fn_calls(); // rust调用其他语言（比如C）
    unsafe_rust::call_from_c(); // 其他语言调用Rust函数
    unsafe_rust::static_variable();
    unsafe_rust::unsafe_trait();
    unsafe_rust::unions();
}
