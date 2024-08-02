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

    let user2 = User{
        email: String::from("another@gmail.com"),
        username: String::from("another"),
        // 表示其他字段的值和user1对应字段的值相同
        ..user1
    };
}
