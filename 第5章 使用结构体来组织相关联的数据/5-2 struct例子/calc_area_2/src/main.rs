fn main() {
    let rect = (30, 50);
    println!("{}", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
