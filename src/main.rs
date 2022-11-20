#![allow(unused)]

mod ch8;
mod ch1;
mod ch2;

use crate::ch8::string;
use crate::ch8::vector;
use crate::ch8::hashmap;
use crate::ch8::exercise;

fn main() {
    ch1();
    ch2();
    // ch8();
}

pub fn ch1() {
    ch1::hello();
}

pub fn ch2() {
    ch2::game();
}

pub fn ch8() {
    ch8::vector();

    ch8::string();

    ch8::hashmap();

    ch8::exercises();
}


