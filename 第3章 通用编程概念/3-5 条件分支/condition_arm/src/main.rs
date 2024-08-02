fn main() {
    let x = 6;

    // Tips: 若代码中出现超过1个if-else 应考虑使用match重构
    if x % 4 == 0 {
        println!("{} is divisible by 4", x);
    } else if x % 3 == 0 {
        println!("{} is divisible by 3", x);
    } else if x % 2 == 0 {
        println!("{} is divisible by 2", x);
    } else {
        println!("{} is not divisible by 4, 3, or 2", x);
    }

    let remainder = x % 4;
    match remainder {
        0 => println!("{} is divisible by 4", x),
        1 => println!("{} is not divisible by 4, 3, or 2", x),
        2 => println!("{} is divisible by 2", x),
        3 => println!("{} is divisible by 3", x),
        _ => println!("{} is not divisible by 4, 3, or 2", x),
    }
}
