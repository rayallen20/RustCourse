fn main() {
    let v1 = vec![1, 2, 3];

    let add_one_closure = |x| {x + 1};

    // collect()方法可以将迭代器转换为集合
    // _表示让编译器推断元素的类型
    let v2: Vec<_> = v1.iter().map(add_one_closure).collect();
    println!("{:?}", v2);
}
