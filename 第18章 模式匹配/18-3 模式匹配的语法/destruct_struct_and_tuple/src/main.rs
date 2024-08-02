struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 右值是一个元组 该元组的第1个元素还是一个元组 该元组的第2个元素是Point结构体
    // 模式匹配时 左值的模式需要与右值相同
    let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10});
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
}
