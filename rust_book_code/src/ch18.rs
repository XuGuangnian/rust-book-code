mod pattern_places;
mod pattern_syntax;

pub fn pattern_places() {
    pattern_places::match_pattern();
    pattern_places::if_let_pattern();
    pattern_places::while_let_pattern();
    pattern_places::for_pattern();
    pattern_places::let_pattern();
    pattern_places::fn_pattern();
}

pub fn refutability() {
    println!("==================== refutability ====================");
    // irrefutable: 函数参数、let 语句和 for 循环只能接受不可反驳的模式
    // refutable: if let 和 while let 表达式可以接受可反驳和不可反驳的模式
    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // this pattern will always match, so the `if let` is useless
    // if let x = 5 {
    //     println!("{}", x);
    // };
}

pub fn pattern_syntax() {
    pattern_syntax::match_literals();
    pattern_syntax::match_named_variables();
    pattern_syntax::multiple_patterns();
    pattern_syntax::match_range();
    pattern_syntax::destructure_struct();
    pattern_syntax::destructure_enum();
    pattern_syntax::destructure_nested_structs_enums_tuples();
    pattern_syntax::ignore_values();
    pattern_syntax::ignore_values_with_range();
    pattern_syntax::match_guard();
    pattern_syntax::at_operator(); // @
}
