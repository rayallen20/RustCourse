trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

// 无需指定关联类型
impl Iterator for Counter {
    // 需要在实现中指定关联类型
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.count)
    }
}

// 不能为同一个类型实现多次相同的trait
impl Iterator for Counter {     // error: conflicting implementations of trait `Iterator` for type `Counter`
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.count.to_string())
    }
}

fn main() {
    println!("Hello, world!");
}
