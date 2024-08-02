enum Coin {
    Penny,
    Nickel,
    Dime,
    // 美国的硬币中 25美分的硬币背面是50个州的图案
    // 每个州发行的25美分硬币背面都不一样
    // 此处Quarter变体是可以存值的 其值的类型为UsState枚举
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 此处的state变量 就是表示枚举Coin的变体Quarter中的值
        Coin::Quarter(state) => {
            // 通过模式匹配提取枚举的值
            println!("{:?}", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("{}", value);
}
