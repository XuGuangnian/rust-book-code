#![allow(unused)]

mod ch1;
mod ch10;
mod ch11;
mod ch13;
mod ch14;
mod ch15;
mod ch16;
mod ch17;
mod ch18;
mod ch19;
mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch6;
mod ch8;
mod ch9;

fn main() {
    // ch1();
    // ch2();
    // ch3();
    // ch4();
    // ch5();
    // ch6();
    // ch7();
    // ch8();
    // ch9();
    // ch10();
    // ch11();
    // ch12();
    // ch13();
    // ch14: https://doc.rust-lang.org/cargo/
    // ch15();
    // ch16();
    // ch17();
    // ch18();
    ch19();
}

pub fn ch1() {
    ch1::hello();
}

pub fn ch2() {
    ch2::game();
}

pub fn ch3() {
    ch3::variables();
    ch3::data_type();
    ch3::func();
    ch3::comments();
    ch3::branches();
    ch3::exercises();
}

fn ch4() {
    ch4::ownership();
    ch4::references();
    ch4::slice_type();
}

fn ch5() {
    ch5::structure();
    ch5::example_structs();
    ch5::method_syntax();
}

fn ch6() {
    ch6::enumerate();
    ch6::match_control_flow();
    ch6::if_let();
}

fn ch7() {
    // 一些实验代码会破坏代码结构，因此移动到 module 包下进行
    // cargo run --bin module
}

pub fn ch8() {
    ch8::vector();
    ch8::string();
    ch8::hashmap();
    ch8::exercises();
}

pub fn ch9() {
    ch9::panic();
    ch9::result();
}

pub fn ch10() {
    ch10::generics();
    ch10::traits();
    ch10::lifetimes();
}

fn ch11() {
    // in lib.rs
    // #[cfg(test)]
    // mod tests {}
}

fn ch12() {
    // minigrep
    // cd ../minigrep 最好在此目录下进行，不然读取文件的目录可能不匹配
}

pub fn ch13() {
    ch13::closures();
    ch13::iterators();
}

pub fn ch14() {
    ch14::crates();
    // 2 packages: adder add_one
}

pub fn ch15() {
    ch15::smart_pointer_box();
    ch15::smart_pointer_rc();
    ch15::interior_mutability();
    ch15::reference_cycles();
}

pub fn ch16() {
    ch16::threads();
    ch16::message_passing();
    ch16::concurrency();
}

pub fn ch17() {
    ch17::oop();
    ch17::oodp();
}

pub fn ch18() {
    ch18::pattern_places();
    ch18::refutability();
    ch18::pattern_syntax();
}

pub fn ch19() {
    ch19::unsafe_rust();
    ch19::advanced_traits();
    ch19::advanced_types();
    ch19::advanced_functions_and_closures();
}
