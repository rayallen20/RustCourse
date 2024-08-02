# PART1. 总体原则

- `panic!`: 用于不可恢复的错误
  - 当你认为你可以代替你的调用者来决定某些错误是不可恢复的场景,你应该使用`panic!`
- `Result`: 用于可恢复的错误
  - 在定义一个可能失败的函数时,优先考虑返回`Result`

# PART2. `panic!`适合的场景

- 编写示例用于演示某些概念时,可以使用`Result.unwrap()`
  - 这里`unwrap()`就像是一个占位符,表示"这里应该有错误处理代码"
- 编写原型代码时,可以使用`Result.unwrap()`或`Result.expect()`
  - 编写原型代码时,还不知道后续要怎么处理这些错误,所以可以先使用`Result.unwrap()`或`Result.expect()`
  - 这时`Result.unwrap()`或`Result.expect()`更像是一种明显的记号,表示"这里应该有错误处理代码",以便后续增加程序健壮性
- 测试代码中,可以使用`Result.unwrap()`或`Result.expect()`
  - 在测试过程中,如果调用某个方法失败了,可以直接认为这个测试就是失败的.
  - 而测试的失败状态正是通过`panic!`来实现的.因此,可以使用`Result.unwrap()`或`Result.expect()`
- 当你比编译器掌握更多信息时,可以使用`Result.unwrap()`

例如:

```rust
use std::net::IpAddr;

fn main() {
    // 这行代码绝对不会panic 因为127.0.0.1是一个有效的IP地址
    // 但编译器不知道这一点 所以这里我们可以写一个unwrap() 但实际上我们自己知道这里绝对不会panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
```

# PART3. 错误处理的指导性建议

- 当代码最终可能处于损坏状态时,最好使用`panic!`
  - 损坏状态(Bad state): 某些假设、保证、约定或不可变性被打破
    - 比如:非法的值、矛盾的值或空缺的值被传入代码
  - 以及下列情况中的一条:
    - 这种损坏状态并不是预期能够偶然发生的情况(换言之,这种损坏状态是个意外.那么这种意外情况下,就该使用`panic!`)
    - 在损坏状态后,代码无法继续运行
    - 在你使用的类型中,没有一个好的方法来将这些损坏状态的信息进行编码

# PART4. 场景建议

- 当用户调用你的代码时,传入了一个无意义的值时,建议使用`panic!`
- 你的代码调用了外部不可控的代码(比如你同事写的代码)时,对端返回了一个非法状态,且你无法修复这个问题时,建议使用`panic!`
- 如果失败是可以预期的,建议使用`Result`
- 当你的代码对值进行操作之前,你应该先验证这些值.如果验证失败,你应该使用`panic!`

# PART5. 为验证创建自定义类型

以最初的猜数字游戏为例,我们可以创建一个自定义类型`Guess`来验证用户输入的数字是否在1-100之间:

```rust
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
```