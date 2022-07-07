pub fn create() {
    let mut s = String::new();
    println!("{:?}", s);

    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{:?}", s);

    let s = String::from("initial contents");
    println!("{:?}", s);

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    println!("{:?}", hello); // 乱码
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

pub fn update() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}

// + : fn add(self, s: &str) -> String;
pub fn combine_two_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}", s3);
}

pub fn combine_multiple_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // too complicated

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}

pub fn not_indexed() {
    let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("hllo").len();
    let len = String::from("Здравствуйте").len();
    println!("{}", len);

}

pub fn slice() {
    let hello = "Здравствуйте";;
    let s = &hello[0..4];
    
    // let s = &hello[0..1];

    // println!("{}", s); // 乱码
}

pub fn iterate() {
    for c in "नमस्ते".chars() {
        println!("{}", c); // 乱码???
    }
}
