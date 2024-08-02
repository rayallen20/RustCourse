fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.push_str(", world!"); // error: cannot borrow `*s` as mutable, as it is behind a `&` reference
    s.len()
}
