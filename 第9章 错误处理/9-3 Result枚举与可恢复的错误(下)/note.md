# PART1. 传播错误

函数将错误返回给调用者,让调用者来处理错误,这种方式称为传播错误.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut opening_file = match f {
        Ok(file) => file,
        // File::open()函数返回的io::Result中,其Err变体的类型为io::Error
        // 和本函数返回的Result的Err变体类型一致
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // read_to_string()方法要求File实例的可变引用 因此需要让 opening_file 变量是可变的
    match opening_file.read_to_string(&mut s) {
        // read_to_string()方法返回的io::Result中,其Ok变体的类型为usize 表示读取的字节数
        Ok(_) => Ok(s),

        // read_to_string()方法返回的io::Result中,其Err变体的类型为io::Error
        // 和本函数返回的Result的Err变体类型一致
        Err(e) => Err(e),
    }
    // 最终返回的是这个match表达式的结果 要么是Ok(s) 要么是Err(e)
}
```

```bash
cargo run
   Compiling error_propagation_example_1 v0.1.0 (/error_propagation_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.42s
     Running `target/debug/error_propagation_example_1`
Error: No such file or directory (os error 2)
```

# PART2. `?`运算符

## 2.1 基本使用

`?`运算符可以简化传播错误的代码.它的含义为:如果是Err,则将Err的值返回(也就是传播给调用者);如果是Ok,则将Ok的值解包(也就是取出Ok的值).

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 这行代码等价于下面的 match 表达式
    let mut opening_file = File::open("hello.txt")?;

    // let mut opening_file = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    // 这行代码等价于下面的 match 表达式
    // 这里我们并没有用到Ok()中的值 所以不需要使用变量接收
    opening_file.read_to_string(&mut s)?;

    // match opening_file.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    Ok(s)
}
```

## 2.2 `?`运算符与`from()`函数

`from()`函数来自于`std::convert::From` trait,它用于将一个错误类型转换为另一个错误类型.

- 被`?`所应用的错误,会隐式地被`from()`函数转换为函数返回类型的错误类型.(`?`运算符会调用`from()`函数)
- 但是,这种转换并不是随意的:
  - 类型`ErrA`想要转换成类型`ErrB`,需要实现`from() -> ErrB`函数.这样在被`?`运算符调用时,`ErrA`才会被隐式地转换为`ErrB`.
- `from()`函数的用途:针对不同的错误原因,返回同一种错误时使用
  - 这要求每个错误类型都实现了`from()`函数,将自己转换为目标错误类型.

## 2.3 `?`运算符与链式调用

刚才的例子可以使用链式调用进一步简化:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## 2.4 `?`运算符只能用于返回类型为`Result`的函数

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

```bash
cargo run
   Compiling question_mark_only_return_result v0.1.0 (/question_mark_only_return_result)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:36
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let f = File::open("hello.txt")?;
  |                                    ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `question_mark_only_return_result` (bin "question_mark_only_return_result") due to 1 previous error
```

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`): `?`运算符只能用于返回类型为`Result`或`Option`的函数(或者其他实现了`FromResidual`的类型)

## 2.5 `?`运算符与`main()`函数

`main()`函数的返回类型为`()`(空元组),但它的返回类型也可以是`Result`:

```rust
use std::error::Error;
use std::fs::File;

// Box<dyn Error> 是一个trait对象, 在这里简单的理解为是任何可能的错误类型即可
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

`Box<dyn Error>`是一个trait对象,这里先不细讲,简单理解为是任何可能的错误类型即可.