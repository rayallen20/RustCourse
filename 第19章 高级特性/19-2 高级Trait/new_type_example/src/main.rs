use std::fmt::Display;

/// 想要为Vector这个外部的类型实现Display 这个外部的Trait
/// 使用new type模式 将Vec<String>包装成一个新的类型Wrapper
/// 然后为Wrapper实现Display
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
