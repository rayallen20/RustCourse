fn main() {
    let s = String::from("hello");
    take_ownership(s); // s是分配在Heap上的,传入函数时所有权发生了移动(move),此后s不能再使用
    // println!("{}", s); // 编译错误: value borrowed here after move

    let x = 5;
    makes_copy(x); // x是i32类型,实现了Copy trait,因此传入函数时传递的是x的副本(传入时发生了copy),而不是x本身,所以x仍然有效
    println!("{}", x);
} // s和x都离开作用域 s由于已经失效,因此不会调用drop函数,而x由于实现了Copy trait,因此没有drop函数

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string离开作用域 调用drop函数释放内存

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer离开作用域,由于实现了Copy trait而不能实现Drop trait,因此不会调用drop函数
