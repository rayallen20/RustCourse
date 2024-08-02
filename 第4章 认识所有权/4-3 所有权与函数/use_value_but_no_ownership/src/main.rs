fn main() {
    let s1 = String::from("hello");

    // step2. s2 接收返回的所有权
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // 实际上s1还是失去了所有权 因此这个例子中函数calculate_length还是获得了s1的数据(hello)的所有权
    // 只是最终通过返回值的方式又将这个数据的所有权还给了调用它的函数
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
}

fn calculate_length(s: String) -> (String, usize) {
    // 这里length是该函数本该返回的计算结果
    let length = s.len();
    // 但是为了保证调用者仍旧保持对s的数据的所有权,因此不得不将s返回
    (s, length)
} // step1. 将 s 的所有权返回给调用它的函数
