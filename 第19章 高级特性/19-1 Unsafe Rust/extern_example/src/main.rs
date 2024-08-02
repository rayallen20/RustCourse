extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        let abs_value = abs(-3);
        println!("Absolute value of -3 according to C: {}", abs_value);
    }
}
