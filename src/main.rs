#![allow(unused)]

mod ch1;
mod ch10;
mod ch11;
mod ch13;
mod ch14;
mod ch15;
mod ch16;
mod ch17;
mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch6;
mod ch7;
mod ch8;
mod ch9;

// inline module
mod ch7_garden_inline {}

// src/ch7_garden.rs
mod ch7_garden;
// src/ch7_garden_mod/ch7_garden_mod
mod ch7_garden_mod;

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

    // ch13();
    // ch14: https://doc.rust-lang.org/cargo/
    // ch15();
    // ch16();
    ch17();
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
    ch7::modules::module();
    ch7::modules::module_path();
    ch7::modules::pub_use();
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

pub fn ch13() {
    ch13::closures();

    ch13::iterators();
}

pub fn ch14() {
    ch14::crates();

    // cargo workspace: https://github.com/XuGuangnian/add
}

pub fn ch15() {
    ch15::smart_pointer();
}

pub fn ch16() {
    ch16::concurrency();
}

pub fn ch17() {
    ch17::oop();
}
