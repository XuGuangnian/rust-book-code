pub fn func_example() {
    println!("==================== func_example ====================");
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

pub fn func_param(x: i32) {
    println!("==================== func_param ====================");
    println!("The value of x is: {x}");
}

pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("==================== print_labeled_measurement ====================");
    println!("The measurement is: {}{}", value, unit_label);
}

pub fn statement() {
    println!("==================== statement ====================");
    let y = 6;
}

pub fn expression() {
    println!("==================== expression ====================");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

pub fn return_value() {
    println!("==================== return_value ====================");
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
