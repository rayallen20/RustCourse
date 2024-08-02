struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1's email: {}", user1.email);
}
