pub(crate) fn immut_variables() {
    let x = 5;
    println!("The value of x is: {x}");
    // cannot assign twice to immutable variable
    // x = 6;
    // println!("The value of x is: {x}");
}

pub(crate) fn mut_variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

pub(crate) fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant is {THREE_HOURS_IN_SECONDS}")
}

pub(crate) fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // change variable data type
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
