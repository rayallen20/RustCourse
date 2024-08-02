fn main() {
    let mut s = String::from("hello world");
    let index = word_index(&s);

    // 获取索引后修改字符串
    s.clear();

    println!("index: {}", index);
    println!("s: {}", s);
}

fn word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    // step1. bytes.iter(): 该方法的返回值类型是一个迭代器
    // step2. enumerate(): 该方法的返回值类型是一个元组.元组的第一个元素是索引,第二个元素是引用(这个引用是不可变的)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
