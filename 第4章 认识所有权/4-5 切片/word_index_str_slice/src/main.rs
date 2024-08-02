fn main() {
    let s = String::from("hello world");
    let index = word_index(&s);

    // 获取索引后修改字符串
    // 注意: &s是一个不可变引用,而s.clear()需要一个可变引用
    // 这违反了借用规则: 在特定作用域中,要么只能有一个可变引用,要么只能有多个不可变引用
    // 这样,我们找到的索引就和字符串产生了关联,确保了我们不会在索引之后修改字符串
    // s.clear(); // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("index: {}", index);
    println!("s: {}", s);
}

fn word_index(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}