fn main() {
    let num = Some(4);

    match num {
        // 匹配num的值为Some<T>变体,且T的值小于5
        Some(x) if x < 5 => println!("less than five: {}", x),
        // 匹配num的值为Some<T>变体
        Some(x) => println!("{}", x),
        None => (),
    }
}
