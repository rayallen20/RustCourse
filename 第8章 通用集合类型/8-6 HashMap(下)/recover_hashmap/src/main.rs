use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    println!("{:?}", map);

    map.insert(String::from("a"), 3);
    println!("{:?}", map);
}
