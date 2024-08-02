use std::collections::HashMap;

fn main() {
    let words = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        // or_insert()方法返回的是一个可变引用 若K存在 则返回对应的V的可变引用
        // 若K不存在 则插入K-V并返回V的可变引用
        let count :&mut i32 = map.entry(word).or_insert(0);

        // 由于count是一个可变引用 所以想要修改它的值需要先解引用
        *count += 1;
    }

    println!("{:?}", map);
}
