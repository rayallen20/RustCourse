fn main() {
    let w = 30;
    let l = 50;
    println!("{}", area(w, l));
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}