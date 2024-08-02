struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        // 此处的 y:0 是一个条件,表示该模式只匹配y值为0的Point结构体
        Point{x, y: 0} => println!("On the x axis at {}", x),

        // 此处的 x:0 是一个条件,表示该模式只匹配x值为0的Point结构体
        Point{x: 0, y} => println!("On the y axis at {}", y),

        Point{x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}
