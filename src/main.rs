#![allow(unused)]

mod ch1;
mod ch10;
mod ch11;
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

    // ch8();
    // ch9();
    // ch10();
    ch11();
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

pub fn ch11() {}
