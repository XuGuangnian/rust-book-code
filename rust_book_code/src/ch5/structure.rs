struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn struct_definition() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

pub fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;

pub fn unit_like_struct() {
    let subject = AlwaysEqual;
}

// struct UserNoLifetime {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

pub fn struct_reference_no_lifetime() {
    // let UserNoLifetime = UserNoLifetime {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // }; // expected named lifetime parameter
}
