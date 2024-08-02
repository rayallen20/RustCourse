use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);

    let e = map.entry(String::from("c"));

    // match 也会获取到e的所有权 所以这里借用e的所有权 确保e不会发生移动
    match &e {
        // entry()方法的返回值为一个Entry枚举类型, 它有两个变体:Occupied和Vacant

        // Occupied表示这个K已经存在 它包含了这个K对应的V
        Entry::Occupied(o) => {
            println!("Occupied: {:?}", o);
        },

        // Vacant表示这个K不存在 它包含了这个K
        Entry::Vacant(v) => {
            println!("Vacant: {:?}", v);
        }
    }

    // or_insert()方法: 如果K存在,则返回对应的V的可变引用;
    // 如果K不存在,则插入K和V,并返回V的可变引用
    e.or_insert(3);
    println!("{:?}", map);

    let e2 = map.entry(String::from("a"));
    e2.or_insert(4);
    println!("{:?}", map);
}
