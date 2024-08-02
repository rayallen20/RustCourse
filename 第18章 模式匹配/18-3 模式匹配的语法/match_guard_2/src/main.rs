fn main() {
    let num = Some(4);

    match num {
        // 匹配num的值为Some<T>变体
        Some(x) => println!("{}", x),
        // 匹配num的值为Some<T>变体,且T的值小于5
        // 此时已经走了Some(x) => println!("{}", x)的分支 因此该分支永远不会生效
        Some(x) if x < 5 => println!("less than five: {}", x),
        None => (),
    }
}
