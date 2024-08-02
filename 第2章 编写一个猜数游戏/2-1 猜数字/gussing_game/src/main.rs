use std::io;

fn main() {
    println!("猜数!");

    let mut guess = String::new();

    // rust中 引用也是不可变的 但是可以通过mut关键字来声明可变引用
    // 因此此处不能写 &guess 而是 &mut guess
    io::stdin().read_line(&mut guess).expect("读取失败");

    println!("你猜的数是: {}", guess);
}
