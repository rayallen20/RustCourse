fn generic<T> (t: T) {}

// 以上函数会被隐式转换为如下形式:
// fn generic<T: Sized> (t: T) {}

fn main() {}
