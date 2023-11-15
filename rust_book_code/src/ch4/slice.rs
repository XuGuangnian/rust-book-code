pub fn first_word_func() {
    let mut s = String::from("hello world");
    let word = first_word_no_slice(&s); // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn slices() {
    println!("==================== slices ====================");
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello: {}", hello);
    let world = &s[6..11];
    println!("world: {}", world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice: {}", slice);
    let slice = &s[..2];
    println!("slice: {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("slice: {}", slice);
    let slice = &s[3..];
    println!("slice: {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("slice: {}", slice);
    let slice = &s[..];
    println!("slice: {}", slice);
}

pub fn first_word_slice() {
    println!("==================== first_word_slice ====================");
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5

    // s.clear(); // 错误
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn array_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
