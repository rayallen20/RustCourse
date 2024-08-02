fn main() {
    let s = String::from("hello world");

    // &s[0..5] 左闭右开区间 [0, 5) 即 h e l l o
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    // 语法糖: 从0开始可以省略0
    let same_hello: &str = &s[..5];

    // 语法糖: 到结尾可以省略结尾
    let same_world: &str = &s[6..];

    // 语法糖: 从0开始到结尾可以省略两端
    let hello_world: &str = &s[..];
}
