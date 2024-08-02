fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里的y是一个新的变量,也就是所谓匹配时命名的变量,而不是match表达式的作用域外边的y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
}
