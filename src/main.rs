#![allow(unused)]

mod ch1;
mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch6;
mod ch8;

use crate::ch8::exercise;
use crate::ch8::hashmap;
use crate::ch8::string;
use crate::ch8::vector;

fn main() {
    ch1();
    ch2();
    ch3();
    ch4();
    ch5();
    ch6();
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
