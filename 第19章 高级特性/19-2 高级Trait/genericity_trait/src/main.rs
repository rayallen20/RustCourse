pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    count: u32,
}

// 需要标注泛型的具型
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(self.count)
    }
}

// 可以为同一个类型多次实现泛型Trait
impl Iterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        let res = self.count.to_string();
        match res {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
