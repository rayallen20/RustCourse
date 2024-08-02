fn main() {
    let v = vec!['a', 'b', 'c'];

    // vec.iter().enumerate()方法作用于迭代器上,该方法将迭代器中的元素转换成一系列元组
    // 每个元组包含2个元素: 第1个元素为索引,第2个元素为值
    // 简言之 vec.iter().enumerate()方法返回的是一个产生这种元组的迭代器
    // 这里for关键字后边的(index, value) 就是要匹配的模式
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
