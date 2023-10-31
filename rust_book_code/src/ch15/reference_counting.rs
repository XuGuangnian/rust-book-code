use crate::ch15::reference_counting::List::{Cons, Nil};
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn cons_list() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // 也可以调用 a.clone(), 不过在这里 Rust 的习惯是使用 Rc::clone
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
