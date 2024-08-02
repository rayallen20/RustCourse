# PART1. 控制流运算符-match

match允许一个值与一系列模式进行匹配,并执行匹配成功的模式对应的代码

其中模式可以为字面值、变量名、通配符等

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 本函数用于将枚举的变体转换为对应的美分值
fn value_in_cents(coin: Coin) -> u8 {
    // 此处的coin就是一个表达式
    match coin {
        // => 左侧是一个待匹配的模式
        // => 右侧是一个表达式 若表达式超过1行 需要使用{}包裹
        // 在match表达式中,会按顺序匹配每一个模式,一旦匹配成功,就会执行对应的表达式,并结束匹配
        // 匹配成功的表达式的值 就是整个match表达式的值
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);
}
```

1. match本身是一个表达式
2. match表达式会按顺序匹配每一个模式,一旦匹配成功,就会执行模式对应的表达式,并结束匹配
3. 匹配成功的表达式的值,就是整个match表达式的值
4. 这也就意味着,match表达式中,各个分支对应的表达式的返回值,其类型必须相同

# PART2. 绑定值的模式

match表达式中,匹配到的分支可以将被匹配对象的部分值绑定到变量上.因此,可以从enum变体中提取值

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    // 美国的硬币中 25美分的硬币背面是50个州的图案
    // 每个州发行的25美分硬币背面都不一样
    // 此处Quarter变体是可以存值的 其值的类型为UsState枚举
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 此处的state变量 就是表示枚举Coin的变体Quarter中的值
        Coin::Quarter(state) => {
            // 通过模式匹配提取枚举的值
            println!("{:?}", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("{}", value);
}
```

# PART3. 匹配Option<T>

回到上一讲的问题,我想让Option<T>与T相加,该如何实现?

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let z = match y {
        Some(i) => i + x,
        // 此处将None视作0 0与x相加结果还是x
        None => x,
    };

    println!("{}", z);
}
```

```bash
 cargo run
   Compiling match_option v0.1.0 (/match_option)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.14s
     Running `target/debug/match_option`
7
```

# PART4. match表达式必须穷举所有可能

match表达式必须穷举所有可能的情况,否则编译器会报错

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let z = match y {
        Some(i) => i + x,
    };

    println!("{}", z);
}
```

```bash
cargo run
   Compiling not_exhaustivity v0.1.0 (/not_exhaustivity)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:5:19
    |
5   |     let z = match y {
    |                   ^ pattern `None` not covered
    |
```

non-exhaustive patterns: `None` not covered: 没有覆盖None的情况

## 4.1 使用_通配符替代其余没有列出的值

```rust
fn main() {
    let v: u8 = 0;
    match v {
        0 => println!("zero"),
        _ => println!("non-zero"),
    }
}
```