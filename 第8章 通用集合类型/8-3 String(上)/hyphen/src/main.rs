fn main() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");

    // + 运算符调用了 add 方法
    // add 方法的签名是 fn add(self, s: &str) -> String
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", s2); // 从方法签名可以看出,s2 的所有权没有被转移,只是发生了借用, 因此 s2 仍然可以使用

    // 而s1的所有权已经被转移,在这里s1已经失效了,所以这里会报错
    println!("{}", s1); // error: value borrowed here after move
}
