fn main() {
    // 最常用的let模式: 一个变量绑定一个值
    let a = 5;
    println!("a: {}", a);

    // 也可以使用模式同时匹配值给多个变量
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 但是 如果左边的模式和右边的值不匹配 编译器会报错
    // let (q, w) = (1, 2, 3);     // error: Type mismatch
}
