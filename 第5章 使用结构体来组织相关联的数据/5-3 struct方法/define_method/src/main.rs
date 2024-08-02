#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl关键字用于定义方法
impl Rectangle {
    // 方法的第1个参数总是self,表示调用该方法的实例
    // &self可以被推断为&Rectangle 且它是一个借用(不可变引用)
    // 这里可以是 self &self &mut self 取决于具体需求
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
    println!("rect is {:#?}", rect);
}