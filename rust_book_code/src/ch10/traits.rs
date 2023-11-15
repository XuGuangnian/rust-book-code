use std::fmt::{Debug, Display};

use rust_book_code::{NewsArticle, Summary, Tweet};

pub fn aggregator() {
    println!("==================== aggregator ====================");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize2());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

pub fn trait_parameter() {
    println!("==================== trait_parameter ====================");
    let mut tweet = Tweet {
        username: "notify".to_string(),
        content: "Summary".to_string(),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    // 参数类型：实现了 trait Summary 的类型
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    tweet.username = "notify_bound".to_string();
    notify_bound(&tweet);
    pub fn notify_bound<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    tweet.username = "notify_two_param".to_string();
    let news_article = NewsArticle {
        headline: "notify_two_param".to_string(),
        location: "".to_string(),
        author: "".to_string(),
        content: "".to_string(),
    };
    notify_two_param(&tweet, &news_article);
    pub fn notify_two_param(item1: &impl Summary, item2: &impl Summary) {
        println!("Breaking news! {}-{}", item1.summarize(), item2.summarize());
    }

    tweet.username = "notify_two_param_same_type".to_string();
    notify_two_param_same_type(&tweet, &tweet);
    pub fn notify_two_param_same_type<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {}-{}", item1.summarize(), item2.summarize());
    }

    tweet.username = "notify_two_trait".to_string();
    notify_two_trait(&tweet);
    pub fn notify_two_trait(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    tweet.username = "notify_two_trait_bound".to_string();
    notify_two_trait_bound(&tweet);
    pub fn notify_two_trait_bound<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        0
    }

    fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        0
    }
}

pub fn trait_return() {
    println!("==================== trait_return ====================");
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // todo in ch17
    // fn returns_summarizable2(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn trait_conditional() {
    println!("==================== trait_conditional ====================");
    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let pair = Pair::new(
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        },
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        },
    );
    // pair.cmp_display();

    // std implements the ToString trait on any type that implements the Display trait
    // impl<T: Display> ToString for T {
    //     // --snip--
    // }
    let s = 3.to_string();
    println!("{}", s)
}
