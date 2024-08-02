fn main() {
    let v :Some(u8)  = Some(0);

    // 仅当v是Some(42)时 进行特殊处理
    // 否则执行else分支
    if let v = Some(42) {
        println!("v: {}", v);
    } else {
        println!("other");
    }

    // 等效于match表达式中 仅匹配1个模式并使用通配符匹配其他模式
    match v {
        Some(42) => println!("v: {}", 42),
        _ => println!("other"),
    }
}
