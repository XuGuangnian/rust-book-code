use std::io;

pub(crate) fn integer() {
    let x: i16 = 10;
    let y = 20;
    let z: isize = 30;
    println!("The integer value of x, y, z is {x}, {y}, {z}");
}

pub(crate) fn float() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("The float value of x, y is {x}, {y}");
}

pub(crate) fn calc() {
    // addition
    let sum = 5 + 10;
    println!("The value of sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {quotient}");

    let floored = 2 / 3; // Results in 0
    println!("The value of floored is {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {remainder}");
}

pub(crate) fn bool() {
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("bool: {t}, {f}");
}

pub(crate) fn char() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("char: {}, {}, {}", c, z, heart_eyed_cat);
}

pub(crate) fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);
}

pub(crate) fn array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3, 3, 3, 3, 3];
    // a more concise way
    let d = [3; 5];

    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);

    let a = [1, 2, 3, 4, 5];

    array_out_of_index();
}

fn array_out_of_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
