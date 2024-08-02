#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
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
    // 通过引用调用方法 和 通过实例调用方法 二者等效
    println!("{}", (&rect).area());
    println!("rect is {:#?}", rect);

    let rect2: &Rectangle = &Rectangle {
        width: 30,
        height: 50,
    };

    // 通过引用调用方法 和 通过实例调用方法 二者等效
    println!("{}", rect2.area());
    println!("rect2 is {:#?}", rect2);
}