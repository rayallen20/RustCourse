fn main() {
    let s = String::from("你好");

    // 以字节为单位遍历字符串
    // bytes()方法返回值是一个迭代器 可用于遍历字符串的字节
    for b in s.bytes() {
        println!("{}", b);
    }
}
