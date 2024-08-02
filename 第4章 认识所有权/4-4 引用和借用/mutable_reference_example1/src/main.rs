fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", s1, s2);
}
