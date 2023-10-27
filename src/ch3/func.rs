pub fn func_example() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

pub fn func_param(x: i32) {
    println!("The value of x is: {x}");
}

pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

pub fn statement() {
    let y = 6;
}

pub fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

pub fn return_value() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}
