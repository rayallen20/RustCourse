fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s4);

    // 等效于使用 format! 宏
    let s1 = String::from("tic");
    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s5);
}
