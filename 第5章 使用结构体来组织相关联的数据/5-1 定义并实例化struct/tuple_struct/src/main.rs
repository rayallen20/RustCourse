struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    // black和origin是不同的类型
    let origin = Point(0, 0, 0);
}
