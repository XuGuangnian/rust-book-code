use crate::ch2;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn type_alias() {
    println!("==================== type_alias ====================");
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}

pub fn never_type() {
    // continue -> !
    /***
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    ch2::game::guessing_game();
     */

    // panic! -> !
    /***
    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }
     */

    // loop -> !
    /***
    print!("forever ");
    loop {
        print!("and ever ");
    }
     */
}

pub fn dyn_sized_types() {
    println!("==================== dyn_sized_types ====================");
    let s1 = "Hello there!";
    let s2 = "How's it going?";
    // 动态大小类型用法：必须将动态大小类型的值置于某种指针之后
    // 如: &str,Box<str>,&dyn Trait,Box<dyn Trait>,Rc<dyn Trait>
    // 通过加上`?Sized` 取消编译时类型大小已知的检查，T置于某种指针（引用）之后
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}
