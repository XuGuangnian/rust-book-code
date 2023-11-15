pub fn match_pattern() {
    println!("==================== match_pattern ====================");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

pub(crate) fn if_let_pattern() {
    println!("==================== if_let_pattern ====================");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

pub fn while_let_pattern() {
    println!("==================== while_let_pattern ====================");
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn for_pattern() {
    println!("==================== for_pattern ====================");
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

pub fn let_pattern() {
    println!("==================== let_pattern ====================");
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); // 错误
    let (x, y, _) = (1, 2, 3);
    let (x, ..) = (1, 2, 3);
}

pub fn fn_pattern() {
    println!("==================== fn_pattern ====================");
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
