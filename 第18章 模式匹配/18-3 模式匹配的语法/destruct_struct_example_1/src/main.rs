struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    // 表示把结构体类型的变量p的
    // x字段的值赋给变量a
    // y字段的值赋给变量b
    let Point{x: a, y: b} = p;
    println!("a = {}, b = {}", a, b);
}
