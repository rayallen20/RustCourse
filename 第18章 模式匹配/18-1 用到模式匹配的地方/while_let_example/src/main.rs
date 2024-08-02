fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 只要 stack.pop() 方法返回的是 Some<T>变体就继续执行循环
    // 若该方法返回None变体则结束循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
