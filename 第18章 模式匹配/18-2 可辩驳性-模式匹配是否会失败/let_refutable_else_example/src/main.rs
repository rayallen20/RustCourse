fn main() {
    let a: Option<i32> = Some(5);
    // 使用else表达式时 要求该表达式的返回值类型和函数或闭包的返回值类型一致
    let Some(x) = a else { () };    // 这里如果走到else分支 则直接返回一个空的元组
    println!("{}", x);
}
