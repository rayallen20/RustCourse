struct Point<T> {
    x: T,
    y: T,
}

// impl<T> 表示代码块中的方法是针对泛型T的,而非某个具型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 也可以针对具体的类型来实现方法
// 针对具型实现方法时, impl关键字后边就不需要再写<T>了
// 注意: 这些方法只属于Point<i32>这个具型,其他类型的Point没有这些方法
impl Point<i32> {
    fn x1(&self) -> i32 {
        self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
