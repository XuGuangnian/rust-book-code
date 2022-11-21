pub(crate) fn rectangles() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

pub(crate) fn rectangles_with_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_with_tuple(rect1)
    );
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub(crate) fn rectangles_with_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub(crate) fn print_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

pub(crate) fn dbg_macro() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1);
}