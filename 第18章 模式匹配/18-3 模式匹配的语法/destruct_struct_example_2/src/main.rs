struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    // 若模式中的变量名和要解构的结构体中的字段名相同,可省略字段名
    let Point{x, y} = p;
    println!("x: {}, y: {}", x, y);
}
