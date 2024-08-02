fn main() {
    let x = 5;
    match x {
        1 ..= 5 => println!("one through five"),    // ..= 表示闭区间 match表达式不允许使用..表示半开区间 而for循环可以
        _ => println!("anything"),
    }

    for i in 1..5 {
        println!("{}", i);
    }

    let x = 'c';
    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
