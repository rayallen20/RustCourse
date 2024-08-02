fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

// 这里的 x: i32 实际上是一个模式
fn foo(x: i32) {

}

// 同理 这里的 &(x, y): &(i32, i32) 也是一个模式
// 表示该函数接收一个元组引用类型的参数
// 同时使用模式匹配将元组的第1个元素绑定到变量x,第2个元素绑定到变量y
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}
