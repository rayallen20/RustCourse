struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 1, z: 2 };
    match origin {
        Point{ x, ..} => println!("x is {}", x),
    }
}
