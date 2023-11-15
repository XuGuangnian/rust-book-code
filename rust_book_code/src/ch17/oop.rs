use rust_book_code::{Button, Draw, Screen};

pub fn encapsulation() {
    /// oop: encapsulation
    println!("==================== encapsulation ====================");
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// user impl trait Draw
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("draw a select box");
    }
}

pub fn trait_object() {
    println!("==================== trait_object ====================");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
