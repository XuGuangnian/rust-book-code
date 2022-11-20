#![allow(unused)]

mod ch1;
mod ch2;
mod ch3;
mod ch8;

use crate::ch8::exercise;
use crate::ch8::hashmap;
use crate::ch8::string;
use crate::ch8::vector;

fn main() {
    // ch1();
    // ch2();
    ch3();
    // ch8();
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
}

pub fn ch8() {
    ch8::vector();

    ch8::string();

    ch8::hashmap();

    ch8::exercises();
}
