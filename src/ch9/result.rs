use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};

pub fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

pub fn recoverable_errors() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

pub fn unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

pub fn unwrap_and_expect() {
    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

pub fn propagate_errors() {
    let result = match read_username_from_file() {
        Ok(s) => s,
        Err(err) => panic!("failed to read username"),
    };

    println!("{}", result);

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

pub fn propagate_errors_shortcut() {
    let result = match read_username_from_file() {
        Ok(s) => s,
        Err(err) => panic!("failed to read username"),
    };

    println!("{}", result);

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}

pub fn shortcut_chains() {
    let result = match read_username_from_file() {
        Ok(s) => s,
        Err(err) => panic!("failed to read username"),
    };

    println!("{}", result);

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}

pub fn read_to_string() {
    let result = match read_username_from_file() {
        Ok(s) => s,
        Err(err) => panic!("failed to read username"),
    };

    println!("{}", result);

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

pub fn last_char() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
