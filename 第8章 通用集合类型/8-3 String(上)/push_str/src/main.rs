fn main() {
    let mut s = String::from("hello");
    let s2: String = String::from(", world!");
    s.push_str(&s2);
    // 此处打印s2是不会报错的 因为push_str()方法不会获取s2的所有权,而是获取了s2的引用
    println!("{}", s2);
}
