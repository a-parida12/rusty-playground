struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // except for the email; copy everything from ..user1
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}",user1.email);
    println!("{}",user2.email);

    // tupled struct define and use
    struct Point(i32, i32, i32);
    let origin = Point(0, 1, 2);

    // accessed by a '.' and index
    println!("{}",origin.1);


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
