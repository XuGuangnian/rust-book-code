mod adv_func_cls;
mod advanced_traits;
mod advanced_types;
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

pub fn advanced_traits() {
    advanced_traits::associated_types();
    advanced_traits::default_generic_type();
    advanced_traits::generic_type_parameter();
    advanced_traits::calls_same_name_methods();
    advanced_traits::calls_same_name_associated_functions(); // 非方法函数，参数值没有self，使用 fully qualified syntax: <Type as Trait>
    advanced_traits::supertraits();
    advanced_traits::newtype();
}

pub fn advanced_types() {
    advanced_types::type_alias();
    advanced_types::never_type();
    advanced_types::dyn_sized_types();
}

pub fn advanced_functions_and_closures() {
    adv_func_cls::function_pointer();
    adv_func_cls::return_closures();
}
