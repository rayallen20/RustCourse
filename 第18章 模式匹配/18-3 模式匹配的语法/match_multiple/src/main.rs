fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),    // 表示匹配1或2
        3 => println!("three"),
        _ => println!("anything"),
    }
}
