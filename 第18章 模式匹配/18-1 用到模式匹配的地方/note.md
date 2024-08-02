# PART1. 模式

- 模式是Rust中的一种特殊语法,用于匹配复杂和简单类型的结构
- 将模式匹配与匹配表达式和其他构造结合使用,可以更好地控制程序的控制流
- 模式由以下元素(的一些组合)组成:
  - 字面值
  - 解构的数组、enum、struct和tuple
  - 变量
  - 通配符
  - 占位符
- 想要使用模式,需要将其与某个值进行比较:
  - 如果模式匹配,就可以在代码中使用这个值的相应部分(或者说匹配的部分)

# PART2. `match`的arm(分支)

形式:

```
match value {
    pattern => expression,
    pattern => expression,
    pattern => expression,
}
```

- match表达式的要求:
  - 详尽(包含所有可能的情况)

- 一个特殊的模式:`_`(通配符)
  - 会匹配到任何东西
  - **不会绑定到变量**(非常重要)
  - 通常用于match的最后一个arm,表示"其他情况"
  - 或者用于忽略某些值

# PART3. 条件`if let`表达式

- `if let`表达式主要是作为一种简短的方式来等价的代替只有1个匹配项的`match`表达式
- `if let`可选的可以拥有`else`分支,包括:
  - `else if`
  - `else if let`
- **但是,`if let`不会检查穷举性**

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {   // 匹配Option<T>的Some(T)值 如果favorite_color是Some(T)变体就执行{}里的代码 如果favorite_color是None变体就跳过这个代码块
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {   // 匹配Result<T, E>的Ok(T)值 如果age是Ok(T)变体就执行{}里的代码 如果age是Err(E)变体就跳过这个代码块
        // 注意: 这个代码块中的age是u8类型的值 不是作用域外的age变量 换言之 这里的age不是外边的Result<u8, _>类型
        if age > 30 {
            println!("Using purple as the background color");   // 最终会打印这句话
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

```
cargo run
   Compiling if_let_example v0.1.0 (/if_let_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.48s
     Running `target/debug/if_let_example`
Using purple as the background color
```

# PART4. `while let`条件循环

- 只要模式继续满足匹配的条件,那它允许`while`循环一直运行

```rust
fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 只要 stack.pop() 方法返回的是 Some<T>变体就继续执行循环
    // 若该方法返回None变体则结束循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

```
cargo run
   Compiling while_let_example v0.1.0 (/while_let_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/while_let_example`
3
2
1
```

# PART5. `for`循环

- `for`循环是Rust中最常见的循环
- `for`循环中,模式就是紧随`for`关键字之后的部分

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];

    // vec.iter().enumerate()方法作用于迭代器上,该方法将迭代器中的元素转换成一系列元组
    // 每个元组包含2个元素: 第1个元素为索引,第2个元素为值
    // 简言之 vec.iter().enumerate()方法返回的是一个产生这种元组的迭代器
    // 这里for关键字后边的(index, value) 就是要匹配的模式
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

```
cargo run
   Compiling for_example v0.1.0 (/for_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/for_example`
a is at index 0
b is at index 1
c is at index 2
```

# PART6. `let`语句

- `let`语句也是模式
- `let PATTERN = EXPRESSION;`
- 其中的`PATTERN`就是模式

```rust
fn main() {
    // 最常用的let模式: 一个变量绑定一个值
    let a = 5;
    println!("a: {}", a);

    // 也可以使用模式同时匹配值给多个变量
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 但是 如果左边的模式和右边的值不匹配 编译器会报错
    // let (q, w) = (1, 2, 3);     // error: Type mismatch
}
```

```
cargo run
   Compiling let_example v0.1.0 (/let_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/let_example`
a: 5
x: 1, y: 2, z: 3
```

# PART7. 函数参数

- 函数参数也可以是模式

```rust
fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

// 这里的 x: i32 实际上是一个模式
fn foo(x: i32) {

}

// 同理 这里的 &(x, y): &(i32, i32) 也是一个模式
// 表示该函数接收一个元组引用类型的参数
// 同时使用模式匹配将元组的第1个元素绑定到变量x,第2个元素绑定到变量y
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}
```

```
cargo run
   Compiling fn_param_example v0.1.0 (/fn_param_example)
...
warning: `fn_param_example` (bin "fn_param_example") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/fn_param_example`
x: 3, y: 5
```