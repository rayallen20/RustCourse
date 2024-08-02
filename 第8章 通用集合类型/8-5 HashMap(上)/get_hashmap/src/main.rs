use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);

    let exist_key = String::from("a");
    // 注意: get()方法接收的参数类型为&K, 返回值类型为Option<&V>
    let exist = map.get(&exist_key);

    match exist {
        Some(v) => println!("exist key: {} value: {}", exist_key, v),
        None => println!("not exist key: {}", exist_key),
    }
}
