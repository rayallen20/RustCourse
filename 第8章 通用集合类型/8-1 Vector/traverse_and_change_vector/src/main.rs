fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        // *: 解引用符 用于取出引用的值
        *i += 50;
    }

    println!("{:?}", v);
}
