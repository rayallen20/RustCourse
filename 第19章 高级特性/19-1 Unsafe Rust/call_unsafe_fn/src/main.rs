fn main() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("This is an unsafe function");
}
