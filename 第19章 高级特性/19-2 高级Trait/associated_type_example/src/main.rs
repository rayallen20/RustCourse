trait Iterator {
    // 标准库中的Iterator Trait大致定义如下
    // 此处的Item就是关联类型 用以指代迭代时 返回的元素类型
    // 这个Item可以认为就是一种类型占位符
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("Hello, world!");
}
