struct User {
    // struct中使用了引用但是没有生命周期参数
    email: &str, // error: expected lifetime parameter
    username: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");
}
