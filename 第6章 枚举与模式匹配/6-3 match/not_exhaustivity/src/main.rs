fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let z = match y {
        Some(i) => i + x,
    };

    println!("{}", z);
}