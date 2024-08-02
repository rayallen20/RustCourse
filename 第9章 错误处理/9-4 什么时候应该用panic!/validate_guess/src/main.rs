#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    // 将验证的过程封装在new()函数中
    // 这样设计的目的在于: 将所有的验证逻辑放在一个地方 以便于维护
    pub fn new(value: i32) -> Guess {
        // 如果用户的输入非法 则panic
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // 提供一个value()函数 用于获取Guess的值
    // 我们并没有将value字段设置为公有 是为了确保在创建Guess实例后 用户无法再将value的值设置为一个非法的值
    // 因此 我们提供一个访问value的函数
    // 这个函数其实就像是一个getter
    // 且这里借用了self的不可变引用 确保调用者在调用了value()函数后 可以继续使用Guess实例
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(55);
    println!("Guess value: {}", guess.value());
    println!("Guess: {:?}", guess);
}
