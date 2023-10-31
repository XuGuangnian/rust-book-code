use crate::ch15::smart_pointer::List::{Cons, Nil};
use std::ops::Deref;

pub fn heap_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("cons_list: {:?}", list);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //  syntax defines an associated type for the Deref trait to use

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello_str(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_string(name: &String) {
    println!("Hello, {}!", name);
}

pub fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello_str(&m); // &MyBox<String> -> &String -> &str
    hello_string(&m); // &MyBox<String> -> &String
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

pub fn std_mem_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
