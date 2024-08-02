fn main() {
    // if let后跟一个不可辩驳的模式,则编译器会发出警告
    // 因为模式不会失败 所以if let是多余的
    if let x = 5 {      // warning: irrefutable if let pattern
        println!("x: {}", x);
    }
}
