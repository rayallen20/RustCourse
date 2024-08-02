fn main() {
    // step1. 想要创建可变引用,则其对应的变量也必须是可变的
    let mut s1 = String::from("hello");

    // step2. 创建引用时,需要使用 &mut 而不是 & 符号来创建一个可变的引用
    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

// step3. 函数参数中,需要使用 &mut 来接收可变引用
fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world!");
    s.len()
}
