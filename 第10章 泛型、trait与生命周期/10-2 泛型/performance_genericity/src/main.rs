// fn main() {
//     let integer: Option<i32> = Some(5);
//     let float: Option<f64> = Some(2.0);
// }

// 编译器在编译这段代码时,会读取Option<T>中使用过的值,在本例中,进而编译器确定了2种类型:

// 泛型定义的展开
enum Option_i32 {
    Some(i32),
    None,
}

// 泛型定义的展开
enum Option_f64 {
    Some(f64),
    None,
}

// 单态化后的main()函数,编译器会将Option<T>中的T替换为具体的类型
fn main() {
    let integer: Option_i32 = Option_i32::Some(5);
    let float: Option_f64 = Option_f64::Some(2.0);
}