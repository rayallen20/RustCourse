fn main() {
    // 使用String类型 在调用前创建一个完整的String切片调用即可
    let s1 = String::from("hello world");
    let index1 = word_index(&s1[..]);
    println!("index1: {}", index1);
    println!("s1: {}", s1);

    // 使用&str类型 直接调用即可
    let s2 = "fuck world";
    let index2 = word_index(s2);
    println!("index2: {}", index2);
    println!("s2: {}", s2);
}

fn word_index(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}