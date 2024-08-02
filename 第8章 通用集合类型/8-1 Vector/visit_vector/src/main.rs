fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引访问
    let third: &i32 = &v[2];

    // get()方法返回一个Option<&T>类型
    match v.get(2) {
        // Some: 存在指定索引的元素
        Some(element) => println!("The third element is {}", element),
        // None: 不存在指定索引的元素
        None => println!("There is no third element."),
    }
}
