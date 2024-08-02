struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 1 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);

    let p2 = Point { x: 1.2, y: 4.4 };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}
