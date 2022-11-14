// Struct example
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct exmaple
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // Initialize a User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Access struct fileds with '.'
    user1.email = String::from("anotherone@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@exmaple.com"),
        sign_in_count: user1.sign_in_count,
    };
    // also we can use .. to iterate through the rest
    // of the data that is not modified
    let user3 = User {
        email: String::from("iterate@example.com"),
        ..user2
    };

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
