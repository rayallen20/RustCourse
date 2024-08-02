use std::fmt;
use std::fmt::Display;

/// 该Trait用于打印一个图形的轮廓 但该Trait要求其实现者必须实现Display Trait
/// 也就是说该Trait的实现必须实现Display Trait
/// :后即为该Trait依赖的Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 具型要想成为OutlinePrint的实现者 必须成为Display Trait的实现者
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
