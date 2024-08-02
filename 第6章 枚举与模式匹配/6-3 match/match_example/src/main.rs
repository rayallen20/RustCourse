enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 本函数用于将枚举的变体转换为对应的美分值
fn value_in_cents(coin: Coin) -> u8 {
    // 此处的coin就是一个表达式
    match coin {
        // => 左侧是一个待匹配的模式
        // => 右侧是一个表达式 若表达式超过1行 需要使用{}包裹
        // 在match表达式中,会按顺序匹配每一个模式,一旦匹配成功,就会执行对应的表达式,并结束匹配
        // 匹配成功的表达式的值 就是整个match表达式的值
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);
}
