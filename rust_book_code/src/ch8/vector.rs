pub fn create() {
    println!("==================== create ====================");
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
}

pub fn update() {
    println!("==================== update ====================");
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
}

pub fn drop() {
    println!("==================== drop ====================");
    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);
    }
    // not found in this scope
    // println!("{:?}", v);
}

pub fn read_element() {
    println!("==================== read_element ====================");
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // &v[2] is a reference to the third element of v
    println!("The third element is {}", third);

    match v.get(2) {
        // v.get(2) is an Option<&i32>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

// Vector 的工作方式：在 vector 的结尾添加新元素时，在没有足够空间的情况下，
// 它会创建一个新的 vector，并将旧 vector 的元素复制到新 vector 中。
// 这时，第一个元素的引用就指向的被释放的内存。这是不被允许的。
pub fn vector_work_type() {
    println!("==================== vector_work_type ====================");
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow occurs here

    // v.push(6); // mutable borrow occurs here

    println!("The first element is {}", first);
}

pub fn iterate() {
    println!("==================== iterate ====================");
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    for i in v {
        // - `v` moved due to this implicit call to `.into_iter()`
        println!("{}", i);
    }
    // println!("{:?}", v);
}

pub fn iterate_mut() {
    println!("==================== iterate_mut ====================");
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}

pub fn store_multiple_types_by_enum() {
    println!("==================== store_multiple_types_by_enum ====================");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
