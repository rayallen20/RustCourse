fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 这里的(1..5)是一个Range类型 表示从1到5(不包括5)
    for number in (1..5).rev() {
        println!("{}!", number);
    }
}
