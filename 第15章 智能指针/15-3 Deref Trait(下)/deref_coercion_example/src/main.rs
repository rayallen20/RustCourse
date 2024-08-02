use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // &m: 其类型为&MyBox<String>
    // 由于MyBox类型实现了Deref trait, 所以Rust会自动调用deref方法, 将&MyBox<String>转换为&String
    // 由于String类型也实现了Deref trait, 所以Rust会再次自动调用deref方法, 将&String转换为&str (String类型的deref方法返回的类型为&str)
    // 这样 就满足hello()函数的参数类型要求了
    hello(&m);

    // 如果Rust没有Deref coercion, 那么上面的代码需要写成下面这样

    // step1. 获得一个&String类型的值
    let s = &(*m);
    // step2. 将&String类型的值转换为&str类型的值
    let str = &s[..];
    hello(str);
}
