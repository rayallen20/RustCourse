# PART1. 例子需求

计算长方形面积

## 1.1 单纯使用2个参数的实现

```rust
fn main() {
    let w = 30;
    let l = 50;
    println!("{}", area(w, l));
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
```

缺点:长和宽这两个参数是没有任何关联的

## 1.2 使用tuple的实现

```rust
fn main() {
    let rect = (30, 50);
    println!("{}", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
```

缺点:通过索引访问元组的元素,不够直观.你不知道`dim.0`和`dim.1`哪个是长哪个是宽

## 1.3 使用struct的实现

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));
}

// 此处area()函数只需借用Rectangle实例即可,而不需要获取该实例的数据所有权
// 因为area()函数不需要修改Rectangle实例的数据,只是需要使用Rectangle实例的数据,所以不需要获取所有权
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

# 1.4 打印Rectangle实例

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));
    
    println!("{}", rect); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

```bash
 cargo run
   Compiling display_struct v0.1.0 (/display_struct)
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
  --> src/main.rs:13:20
   |
13 |     println!("{}", rect); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
   |                    ^^^^ `Rectangle` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `display_struct` (bin "display_struct") due to 1 previous error
```

error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`:`Rectangle`不能使用默认格式化器进行格式化.因为`Rectangle`没有实现`std::fmt::Display` trait

`println!("{}", rect);`中的`{}`是使用默认格式化器进行格式化的,而`Rectangle`没有实现`std::fmt::Display` trait,所以不能使用默认格式化器进行格式化

note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead:在格式化字符串中,你可以使用`{:?}`(或者`{:#?}`进行漂亮打印)代替

尝试使用`{:?}`:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));

    println!("{:?}", rect); // error: `Rectangle` doesn't implement `std::fmt::Debug`
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

```bash
cargo run
   Compiling display_struct v0.1.0 (/display_struct)
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> src/main.rs:13:22
   |
13 |     println!("{:?}", rect); // error: `Rectangle` doesn't implement `std::fmt::Debug`
   |                      ^^^^ `Rectangle` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Rectangle` with `#[derive(Debug)]`
   |
1  + #[derive(Debug)]
2  | struct Rectangle {
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `display_struct` (bin "display_struct") due to 1 previous error
```

error[E0277]: `Rectangle` doesn't implement `Debug`:`Rectangle`不能使用`{:?}`进行格式化.因为`Rectangle`没有实现`Debug` trait

note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`: 给`Rectangle`添加`#[derive(Debug)]`注解,或者手动为`Rectangle`实现`Debug` trait

添加注解:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));

    println!("{:?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

```bash
cargo run
   Compiling display_struct v0.1.0 (/display_struct)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/display_struct`
1500
Rectangle { width: 30, height: 50 }
```

Rust提供了打印调试信息(Debug)的功能,但是我们必须为自己的struct显式地添加`#[derive(Debug)]`注解,也就是选择使用Debug的功能,才能使用`{:?}`进行格式化打印

# 1.5 更清晰地打印Rectangle实例

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));

    println!("{:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

```bash
cargo run
   Compiling display_struct v0.1.0 (/display_struct)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/display_struct`
1500
Rectangle {
    width: 30,
    height: 50,
}
```

`{:#?}`是漂亮打印,会在每个字段的前面加上字段名、换行、缩进,更清晰地打印出struct实例