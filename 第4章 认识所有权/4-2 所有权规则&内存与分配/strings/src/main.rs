fn main() {
    // :: 表示from()是String这个类型下(或者说是String这个模块下)的一个函数
    let mut s = String::from("hello");

    // 这类字符串就可以被修改
    s.push_str(", world!");
    println!("{}", s);
} // s离开作用域,Rust自动调用drop()函数,释放内存
