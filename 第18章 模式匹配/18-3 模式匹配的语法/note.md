# PART1. 匹配字面值

模式可直接匹配字面值

```rust
fn main() {
    let x = 1;
    
    match x { 
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

# PART2. 匹配命名变量

命名的变量是可以匹配任何值的不可辩驳模式

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里的y是一个新的变量,也就是所谓匹配时命名的变量,而不是match表达式的作用域外边的y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
}
```

```
cargo run
   Compiling match_named_variable v0.1.0 (/match_named_variable)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.44s
     Running `target/debug/match_named_variable`
Matched, y = 5
At the end: x = Some(5), y = 10
```

# PART3. 多重模式

- 在`match`表达式中,使用`|`运算符可以匹配多个模式
  - `|`在这里是或的意思

```rust
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),    // 表示匹配1或2
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

# PART4. 使用`..=`匹配范围

`..=`可以作用于数字或字符,表示一个范围

```rust
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
```

```
cargo run
   Compiling match_range v0.1.0 (/match_range)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/match_range`
one through five
1
2
3
4
early ASCII letter
```

# PART5. 解构以分解值

- 可以使用模式来解构struct、enum、tuple,从而引用这些类型值中的部分数据

## 5.1 解构struct

### 5.1.1 解构struct的基本写法

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    // 表示把结构体类型的变量p的
    // x字段的值赋给变量a
    // y字段的值赋给变量b
    let Point{x: a, y: b} = p;
    println!("a = {}, b = {}", a, b);
}
```

```
cargo run
   Compiling destruct_struct_example_1 v0.1.0 (/destruct_struct_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/destruct_struct_example_1`
a = 1, b = 2
```

### 5.1.2 解构struct的简单写法

如果模式中的变量名和要解构的结构体的字段名相同,可以省略字段名

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    // 若模式中的变量名和要解构的结构体中的字段名相同,可省略字段名
    let Point{x, y} = p;
    println!("x: {}, y: {}", x, y);
}
```

```
cargo run
   Compiling destruct_struct_example_2 v0.1.0 (/destruct_struct_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/destruct_struct_example_2`
x: 1, y: 2
```

### 5.1.3 使用`match`解构struct

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        // 此处的 y:0 是一个条件,表示该模式只匹配y值为0的Point结构体
        Point{x, y: 0} => println!("On the x axis at {}", x),

        // 此处的 x:0 是一个条件,表示该模式只匹配x值为0的Point结构体
        Point{x: 0, y} => println!("On the y axis at {}", y),

        Point{x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}
```

```
cargo run
   Compiling match_destruct_struct_example v0.1.0 (/match_destruct_struct_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/match_destruct_struct_example`
On the y axis at 7
```

## 5.2 解构enum

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },    // Move变体的类型为匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        
        // 匹配Move变体时,将匿名结构体的字段解构到x和y变量中
        // 和解构结构体一样,若模式中的变量名和结构体的字段名相同,则可以省略字段名
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        
        // 匹配ChangeColor变体时,将元组的字段解构到r g b这3个变量中
        // 和匹配元组的用法是相同的
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
    }
}
```

## 5.3 解构嵌套的struct和enum

```rust
enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        // 和解构类型为匿名结构体的变体一样,模式也是逐层解构的
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue: {}, saturation: {}, value: {}", h, s, v);
        }
        _ => (),
    }
}
```

## 5.4 解构struct和tuple

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 右值是一个元组 该元组的第1个元素还是一个元组 该元组的第2个元素是Point结构体
    // 模式匹配时 左值的模式需要与右值相同
    let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10});
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
}
```

# PART6. 在模式中忽略值

- 有几种方式可以再模式中忽略整个值或部分值
  - `_`: 忽略整个值
  - `_`配合其他模式可以忽略部分值
  - 使用以`_`开头的变量名可以忽略未使用的变量
  - `..`: 忽略值的剩余部分

## 6.1 使用`_`忽略整个值

```rust
fn main() {
    foo(3, 4);
}

// 函数签名中的下划线表示忽略该参数
fn foo(_: i32, y: i32) {
    println!("{}", y);
}
```

## 6.2 使用嵌套的`_`来忽略值的一部分

```rust
fn main() {
  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  // 本例中match表达式匹配的是一个元组 该元组的类型为 (Option<i32>, Option<i32>)
  // match表达式匹配实现了Copy Trait的类型时,不会获取数据的所有权,而是复制数据
  // match表达式如果匹配引用,则不会获取数据的所有权
  // 但是如果match表达式匹配的类型没有实现Copy Trait,则会获取数据的所有权
  // 本例中,setting_value和new_setting_value都是Option<i32>类型,没有实现Copy Trait
  // 所以match表达式会获取数据的所有权
  match (setting_value, new_setting_value) {
    // 匹配元组中2个元素的值均为Some变体的模式
    // 但该模式并不关心Some中的值是什么 因此可以使用_忽略模式中的部分值
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }

    // 匹配元组中2个元素的值不全是Some变体的模式
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);
}
```

```rust
fn main() {
    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        // 使用_忽略元组中的第2个和第4个元素 仅匹配第1个 第3个和第5个元素
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}
```

## 6.3 使用以`_`开头的变量名忽略未使用的变量

```rust
fn main() {
    // let x = 5; // 编译器会警告x is never used
    let _x = 5; // 加上下划线前缀,编译器就不会警告了
}
```

```rust
fn main() {
  let s = Some(String::from("hello"));

  // 和match表达式相同, if let表达式针对没有实现Copy Trait的类型也会获得其数据的所有权
  // 这里s的所有权被移动到_s上,因此s不能再使用了
  if let Some(_s) = s {
    println!("found a string");
  }

  // println!("{:?}", s); // error: use of moved value: `s`
}
```

但是,如果把模式匹配中的`_s`改为`_`,则`if let`表达式就不会获得`s`的所有权,因为`_`匹配的是一个不可辩驳的模式,不会绑定到任何值上

```rust
fn main() {
    let s = Some(String::from("hello"));

    // 和match表达式相同, if let表达式针对没有实现Copy Trait的类型也会获得其数据的所有权
    // 但是,由于使用了_表示忽略所有值,也就是不会发生绑定的操作,所以不会发生所有权转移
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```

## 6.4 使用`..`忽略值的剩余部分

### 6.4.1 忽略结构体的剩余部分

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 1, z: 2 };
    match origin {
        Point{ x, ..} => println!("x is {}", x),
    }
}
```

### 6.4.2 忽略元组的剩余部分

```rust
fn main() {
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
```

注意:`..`在匹配元组的模式中只能出现1次

```rust
fn main() {
    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        // 这里编译器无法确定该模式究竟要匹配哪个元素
        (.., second, ..) => {   // error: `..` can only be used once per tuple or tuple struct pattern
            println!("{}", second);
        }
    }
}
```

# PART7. 使用`match`守卫来提供额外的条件

## 7.1 `match`守卫

- `match`守卫:即`match`分支的模式后边额外的`if`条件,想要匹配该模式,则这个条件也必须满足
- `match`守卫适用于比单独的模式更复杂的场景

```rust
fn main() {
    let num = Some(4);

    match num {
        // 匹配num的值为Some<T>变体,且T的值小于5
        Some(x) if x < 5 => println!("less than five: {}", x),
        // 匹配num的值为Some<T>变体
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

```
cargo run
   Compiling match_guard_1 v0.1.0 (/match_guard_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/match_guard_1`
less than five: 4
```

但是要注意`match`守卫与各个分支之间的顺序关系

```rust
fn main() {
    let num = Some(4);

    match num {
        // 匹配num的值为Some<T>变体
        Some(x) => println!("{}", x),
        // 匹配num的值为Some<T>变体,且T的值小于5
        // 此时已经走了Some(x) => println!("{}", x)的分支 因此该分支永远不会生效
        Some(x) if x < 5 => println!("less than five: {}", x),
        None => (),
    }
}
```

```
cargo run
   Compiling match_guard_2 v0.1.0 (/match_guard_2)
warning: unreachable pattern
 --> src/main.rs:9:9
  |
9 |         Some(x) if x < 5 => println!("less than five: {}", x),
  |         ^^^^^^^
  |
  = note: `#[warn(unreachable_patterns)]` on by default

warning: `match_guard_2` (bin "match_guard_2") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/match_guard_2`
4
```

## 7.2 `match`守卫不会引入新变量

`match`守卫不是模式,因此不会引入新的变量

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 这里的守卫是 if n == y
        // 但是守卫并不是模式 所以守卫不会引入新的变量
        // n这个变量是在模式Some(n)中引入的
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
}
```

## 7.3 多重模式与`match`守卫结合使用

```rust
fn main() {
    let x = 4;
    let y = false;
    
    match x { 
        4 | 5 | 6 if y => println!("yes"), 
        _ => println!("no"),
    }
}
```

# PART9. `@`绑定

- `@`符号可以让我们创建一个变量,该变量可以在测试某个值是否与模式匹配的同时保存这个值

```rust
enum Message {
  Hello {id: i32}
}

fn main() {
  let msg = Message::Hello {id: 5};

  match msg {
    // 该模式要求匿名结构体的id字段的值在3到7之间 若匹配成功 则将匹配到的id字段值绑定到id_variable变量上
    Message::Hello {id: id_variable @ 3 ..= 7} => {
      println!("Found an id in range: {}", id_variable)
    },

    // 该模式要求匿名结构体的id字段的值在10到12之间 但是没有绑定值到变量上 因此该分支内无法访问到id字段的值
    Message::Hello {id: 10 ..= 12} => {
      println!("Found an id in another range")
    },

    // 该模式对id字段的值没有任何限制 仅将匹配到的id字段值绑定到变量id上
    Message::Hello {id} => {
      println!("Found some other id: {}", id)
    }
  }
}
```

```
cargo run         
   Compiling at_bound v0.1.0 (/at_bound)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/at_bound`
Found an id in range: 5
```