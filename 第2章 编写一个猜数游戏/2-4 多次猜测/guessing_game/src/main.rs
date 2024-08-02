use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密数字是: {}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字!");
                continue;
            },
        };
        println!("你猜的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你输入的数字太小了!"),
            Ordering::Greater => println!("你输入的数字太大了!"),
            Ordering::Equal => {
                println!("你猜对了!");
                break;
            },
        }
    }
}
