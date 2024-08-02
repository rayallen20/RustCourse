fn main() {
    let mut x = Box::new(5);
    println!("x = {}", x);

    let mutable_value = &mut *x;
    *mutable_value += 1;
    println!("x = {}", x);
}
