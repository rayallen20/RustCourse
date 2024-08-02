enum Message {
    Quit,
    Move { x: i32, y: i32 },    // Move变体的类型为匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },

        // 匹配Move变体时,将匿名结构体的字段解构到x和y变量中
        // 和解构结构体一样,若模式中的变量名和结构体的字段名相同,则可以省略字段名
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },

        Message::Write(text) => {
            println!("Text message: {}", text);
        },

        // 匹配ChangeColor变体时,将元组的字段解构到r g b这3个变量中
        // 和匹配元组的用法是相同的
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
    }
}
