use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// 需求1: 为用户自定义的类型添加 #[derive(HelloMacro)] 宏
// 即可为该类型添加HelloMacro宏的默认实现
#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Apples;

// impl HelloMacro for Pancakes {
//     // 需求2: 在默认实现中,要求打印出 "Hello, Macro! My name is XXX!"
//     // 其中XXX为用户自定义的类型名称
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
    Apples::hello_macro();
}

// 大致实现完成的效果如下:
// #[derive!(HelloMacro)]
// struct Pancakes;
// fn main() {
//     Pancakes::hello_macro();    // Hello, Macro! My name is Pancakes!
// }