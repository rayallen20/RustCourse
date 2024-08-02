fn main() {
    print!("forever ");

    // 这里的loop表达式返回的类型即为`!`
    loop {
        // 无限循环
        print!("and ever ");
    }
}
