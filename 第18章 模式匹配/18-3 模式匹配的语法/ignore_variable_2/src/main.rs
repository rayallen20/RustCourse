fn main() {
    let s = Some(String::from("hello"));

    // 和match表达式相同, if let表达式针对没有实现Copy Trait的类型也会获得其数据的所有权
    // 这里s的所有权被移动到_s上,因此s不能再使用了
    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s); // error: use of moved value: `s`
}
