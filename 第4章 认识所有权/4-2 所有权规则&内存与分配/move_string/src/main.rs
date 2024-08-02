fn main() {
    let s1 = String::from("hello");
    // 此时s1的所有权被转移给了s2
    let s2 = s1;

    println!("{}", s1);
}
