enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        // 和解构类型为匿名结构体的变体一样,模式也是逐层解构的
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue: {}, saturation: {}, value: {}", h, s, v);
        }
        _ => (),
    }
}
