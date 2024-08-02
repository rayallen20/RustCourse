fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let z = match y {
        Some(i) => i + x,
        // 此处将None视作0 0与x相加结果还是x
        None => x,
    };

    println!("{}", z);
}
