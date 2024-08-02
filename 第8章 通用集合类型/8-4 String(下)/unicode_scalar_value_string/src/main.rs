fn main() {
    let s = String::from("你好");

    // chars() 方法返回一个迭代器，迭代器的元素是字符串的 Unicode 标量值
    for c in s.chars() {
        println!("{}", c);
    }
}
