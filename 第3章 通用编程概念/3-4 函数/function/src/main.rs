fn main() {
    let y = {
        let x = 3;
        // x + 1 是表达式 而 x + 1; 是语句
        // 一个块的最后一个表达式就是这个块的返回值
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_five(10);
    println!("The value of x is: {}", x);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}