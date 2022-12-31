use rust_book_code::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_book_code::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
