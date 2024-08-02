struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));
}

// 此处area()函数只需借用Rectangle实例即可,而不需要获取该实例的数据所有权
// 因为area()函数不需要修改Rectangle实例的数据,只是需要使用Rectangle实例的数据,所以不需要获取所有权
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
