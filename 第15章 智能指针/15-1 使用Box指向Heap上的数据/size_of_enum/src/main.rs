use std::mem;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("size of Message: {}", mem::size_of::<Message>());
}
