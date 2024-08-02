struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 一旦struct的实例是可变的,那么实例中的所有字段都是可变的
    let mut user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("example1@gmail.com");
    println!("{}", user1.email);
}
