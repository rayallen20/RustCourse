static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 修改静态变量是不安全的操作
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    // 访问静态变量同样也是不安全的操作
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
