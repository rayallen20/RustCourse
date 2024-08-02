fn main() {
    let v1 = vec![1, 2, 3];

    // v1_iter就是一个迭代器 但到这行代码为止 这个迭代器还没有被使用
    // 因此这个迭代器还没有任何效果
    let v1_iter = v1.iter();

    // 此处才开始使用迭代器
    // for循环会取得迭代器的所有权 并在循环内部将迭代器修改为可变的
    // 因此使用for循环遍历迭代器时 不需要加mut关键字
    for element in v1_iter {
        println!("Got: {}", element);
    }
}
