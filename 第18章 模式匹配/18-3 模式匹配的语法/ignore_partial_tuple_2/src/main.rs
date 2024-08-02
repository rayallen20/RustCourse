fn main() {
    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        // 这里编译器无法确定该模式究竟要匹配哪个元素
        (.., second, ..) => {   // error: `..` can only be used once per tuple or tuple struct pattern
            println!("{}", second);
        }
    }
}
