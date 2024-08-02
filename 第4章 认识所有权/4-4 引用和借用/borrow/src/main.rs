fn main() {
    let s1 = String::from("hello");
    // step1. 调用函数时,并没有将s1的数据所有权移动给函数;而是将s1的引用传递给函数
    // Tips: &符号表示引用,引用允许你使用值但不获取其所有权
    let len = calculate_length(&s1);

    // step2. s1的所有权并没有被移动,所以可以继续使用
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域,但是由于s没有数据所有权,所以不会调用drop函数
