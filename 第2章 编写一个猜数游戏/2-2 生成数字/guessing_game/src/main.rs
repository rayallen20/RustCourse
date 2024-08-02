use std::io;
use rand::Rng;

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密数字是: {}", secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取失败");
    println!("你猜的数是: {}", guess);
}
