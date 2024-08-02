enum Message {
    Quit,
    Move { x: i32, y: i32 },
    // 这里Write的类型是String 不是tuple
    // 如果想要tuple类型(只有1个String的tuple) 那么应该写成Write(String, )
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0, 0, 0);

    m.call();
}
