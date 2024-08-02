fn main() {
    let address = 0x12345usize;
    let r = address as *const i32;

    unsafe {
        println!("r is: {}", *r);
    }
}
