fn main() {
    let a: Option<i32> = Some(5);
    if let Some(x) = a {
        println!("{}", x);
    }
}
