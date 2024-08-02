fn main() {
    let mut num = 5;

    // 注意: 这段不安全代码并没有存在于unsafe代码块中
    // 但是 只能在unsafe代码块中对原始指针进行解引用
    // 这2个原始指针是来自一个有效的引用(&num) 所以我们可以确定这2个指针也是有效的
    // 但原始指针并不总是有效的 所以对原始指针进行解引用的操作必须放在unsafe代码块中
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // 对原始指针的解引用需要放在unsafe代码块中
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
