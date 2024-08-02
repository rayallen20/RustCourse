fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 这里的守卫是 if n == y
        // 但是守卫并不是模式 所以守卫不会引入新的变量
        // n这个变量是在模式Some(n)中引入的
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
}
