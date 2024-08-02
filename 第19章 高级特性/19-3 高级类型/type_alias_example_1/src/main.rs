type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    // 类型别名可以与原类型交互 因为类型别名仅仅只是一个别名 不是一个新的类型
    println!("x + y = {}", x + y);
}
