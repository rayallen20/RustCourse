fn main() {
    let x = 4;

    // 此处闭包的函数体中的x不是函数的参数,而是和闭包定义在同一个作用域内的变量
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
