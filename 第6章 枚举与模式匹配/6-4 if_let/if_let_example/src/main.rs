fn main() {
    let v :Some(u8)  = Some(0);

    // 仅当v是Some(42)时 才会执行println!语句
    if let v = Some(42) {
        println!("v: {}", v);
    }

    // if let与只匹配1个模式的match等效
    match v {
        Some(42) => println!("v: {}", 42),
        _ => (),
    }
}
