struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // mixup()方法的泛型参数V, 和Point的泛型参数T没有关系(也就是说这二者的类型可能相同,也可能不同)
    // 同理, W和U也没有关系

    // 参数列表中的self是Point<T, U>类型的, 而other是Point<V, W>类型的 这二者的泛型参数是不同的

    // 返回值列表中的Point<T, W>中的W是和other的泛型参数W相同的, 而T和U是和self的泛型参数T和U相同的
    // 也就是说,泛型方法返回的泛型Point,可以和Struct定义的泛型Point是不同的
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}
