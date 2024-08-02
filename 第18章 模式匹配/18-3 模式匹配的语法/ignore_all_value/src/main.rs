fn main() {
    foo(3, 4);
}

// 函数签名中的下划线表示忽略该参数
fn foo(_: i32, y: i32) {
    println!("{}", y);
}
