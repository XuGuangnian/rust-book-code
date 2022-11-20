pub(crate) fn references() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub(crate) fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

pub(crate) fn data_race() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    // 解除注释后会出现数据竞争
    // println!("{}, {}, and {}", r1, r2, r3);
}

pub(crate) fn dangling_references() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s
    s
}
