fn main() {
    let s = Some(String::from("hello"));

    // 和match表达式相同, if let表达式针对没有实现Copy Trait的类型也会获得其数据的所有权
    // 但是,由于使用了_表示忽略所有值,也就是不会发生绑定的操作,所以不会发生所有权转移
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
