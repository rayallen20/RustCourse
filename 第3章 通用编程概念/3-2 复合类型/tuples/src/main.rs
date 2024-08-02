fn main() {
    // tuple长度固定 一旦声明就无法改变
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);

    // 解构 即获取tuple中的值
    // 等号左侧表示声明一个元组 元组中有3个变量 x, y, z 分别对应tuple中的3个值
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
}
