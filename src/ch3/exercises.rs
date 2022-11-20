use std::io;

pub(crate) fn temperature_convert() {
    // Loop until person asks to quit.
    'main_loop: loop {
        // Get and validate the input.
        let c = 'get_input: loop {
            println!("\nTemperature in celsius ('quit' to end): ");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Error getting input from system.");

            // Remove additional whitespace, including the
            // newline from pressing <enter>.
            let input = input.trim();

            if input == "quit" {
                println!("Goodbye!");
                break 'main_loop;
            };

            match input.parse() {
                Ok(value) => break value,
                Err(_) => {
                    println!("Invalid entry, please try again.");
                    continue 'get_input;
                }
            }
        };

        // Carry out the conversion.
        let f = celsius_to_fahrenheit(c);

        println!("Fahrenheit: {}", f);
    }
}

// Converts Celsius to Fahrenheit.
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

pub(crate) fn fibonacci() {
    loop {
        println!("\nWhich Fibonacci number? (max: 185; ‘quit’ to exit)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: stdin.");

        let trimmed_input = input.trim();

        if trimmed_input == "quit" {
            break;
        }

        let n: u128 = trimmed_input.parse().expect("Error: not a number.");

        // Make sure we don’t overflow u128.
        if n > 185 {
            println!("Sorry, this app can only calculate the Fibonacci sequence up to the 185th position.");
            continue;
        }

        // Display the result.
        println!("Fibonacci number at position {} is {}.", n, f(n));
    }
}

fn f(n: u128) -> u128 {
    let mut prev = 0;
    let mut current = 1;
    for _ in 0..n {
        let next = prev + current;
        prev = current;
        current = next;
    }
    return prev;
}

pub(crate) fn print_lyrics() {
    // Display the lyrics for the twelve days.
    for day in 0..12 {
        display_lyrics_for_day(day);
    }
}

fn display_lyrics_for_day(day: u32) {
    let mut days_to_presents = vec![
        ("first", "And a partridge in a pear tree."),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eight", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    let details = days_to_presents[day as usize];

    let day_in_english = details.0;
    let present_for_day_in_english: &str;

    if day == 0 {
        // Special case lyrics for first day.
        present_for_day_in_english = "A partridge in a pear tree.";
    } else {
        present_for_day_in_english = details.1;
    }

    println!(
        "\nOn the {} day of Christmas, my true love sent to me\n{}",
        day_in_english, present_for_day_in_english
    );

    days_to_presents.reverse();

    for (index, details) in days_to_presents.iter().enumerate() {
        if index > days_to_presents.len() - day as usize - 1 {
            println!("{}", details.1);
        }
    }
}
