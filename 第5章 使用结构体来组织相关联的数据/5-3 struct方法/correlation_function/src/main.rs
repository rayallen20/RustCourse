#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 关联函数使用::调用
    let square = Rectangle::square(3);
    println!("{:#?}", square.width);
    println!("{:#?}", square.height);
}