use std::collections::HashMap;

// pub, pub(crate)的区别: https://cheats.rs/#organizing-code
// pub "Public if parent path is public" visibility for T.
// pub(crate) Visible at most in current crate.
#[allow(clippy::useless_vec)]
pub fn create() {
    println!("==================== create ====================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

pub fn ownership() {
    println!("==================== ownership ====================");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are moved to the map
    println!("{:?}", map);
    // println!("{:?}", field_name); // field_name is no longer valid here
}

pub fn access_values() {
    println!("==================== access_values ====================");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score); // Some(&10)
}

pub fn iterate() {
    println!("==================== iterate ====================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn update() {
    println!("==================== update ====================");
    // overwrite an existing value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only inserting a value if the key is not present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value's presence
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
