struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email = String::from("example@gmail.com");
    let username = String::from("example");
    let mut user1 = build_user(email, username);

    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        // 字段名和变量名均为 email 此时可以简写
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}