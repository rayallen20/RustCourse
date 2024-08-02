use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 无论泛型T为何种具型 均实现了关联函数new()
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 为实现了Display和PartialOrd的泛型T实现cmp_display()方法
// 所谓有条件的实现方法,是指只有当泛型T满足条件:T既实现了Display Trait,又实现了PartialOrd Trait时
// 才为Pair<T>实现cmp_display()方法
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
