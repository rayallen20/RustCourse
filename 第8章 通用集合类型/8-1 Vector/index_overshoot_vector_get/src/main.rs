fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match v.get(100) {
        Some(element) => println!("Item 100 is {}", element),
        None => println!("Sorry, this vector is too short.")
    }
}
