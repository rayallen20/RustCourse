fn main() {
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
