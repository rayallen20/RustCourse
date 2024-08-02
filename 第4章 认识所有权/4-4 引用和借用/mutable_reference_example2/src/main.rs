fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }

    // 这里由于r1的作用域已经结束,所以可以创建新的可变引用
    let r2 = &mut s;
    println!("{}", r2);
}
