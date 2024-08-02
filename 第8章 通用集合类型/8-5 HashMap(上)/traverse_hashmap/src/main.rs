use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);

    // 这里如果不加上& 后续就不能再使用这个map了 因为它的所有权发生了移动
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 加了&后就可以继续使用map
    println!("{:?}", map); // error: borrow of moved value: `map`
}
