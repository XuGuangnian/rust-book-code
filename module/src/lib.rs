pub use crate::front_of_house::hosting;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // pub use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // pub toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // seasonal_fruit 不能访问非pub属性
    // meal.seasonal_fruit = String::from("blueberries");

    println!("{:?}", back_of_house::Appetizer::Soup);
    println!("{:?}", back_of_house::Appetizer::Salad);
}

fn deliver_order() {
    println!("deliver_order");
}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super
        super::deliver_order();
    }

    fn cook_order() {}

    // 这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
    #[allow(dead_code)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 将枚举设为公有，则它的所有成员都将变为公有
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
