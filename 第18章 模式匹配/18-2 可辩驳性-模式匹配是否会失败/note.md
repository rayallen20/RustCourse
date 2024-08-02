# PART1. 模式的两种形式

- 模式有2种形式: 可辨驳的(refutable)、不可辨驳的(irrefutable) (也称可反驳的、不可反驳的 或 可失败的、不可失败的)
- 能够匹配任何可能传递的值的模式: 不可辨驳的模式
  - 例如: `let x = 5;`
  - 这种模式不会失败,因为它能够匹配任何传递给它的值
  - 你可以理解为,把`5`换成其他的值,这个模式也能完成匹配,最终把右值赋给左值(也就是变量`x`)
- 对于某些可能的值,无法进行匹配的模式: 可辨驳的模式
  - 例如: `let Some(y) = a_value;`
  - 这种模式可能会失败,因为当`a_value`的值为`None`变体时,该模式无法匹配
- 函数参数、`let`语句、`for`循环只接受不可辩驳的模式
- `if let`语句、`while let`语句可接受可辩驳的和不可辩驳的模式
  - 实际上对于`if let`语句和`while let`语句,在接收不可辩驳的模式时,编译器会发出警告
  - 因为这种情况下,模式匹配总是成功的,所以使用`if let`语句和`while let`语句这种可辩驳的模式是没有意义的

```rust
fn main() {
    let a: Option<i32> = Some(5);
    
    // Some(x)是一个可辨驳的模式 因为无法匹配None变体
    // 而 let 语句要求模式是不可辨驳的
    let Some(x) = a; // error: refutable pattern in local binding: `None` not covered
}
```

```
cargo run
   Compiling let_refutable_example v0.1.0 (/let_refutable_example)
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = a;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
  = note: the matched value is of type `Option<i32>`
help: you might want to use `let else` to handle the variant that isn't matched
  |
3 |     let Some(x) = a else { todo!() };
  |                     ++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `let_refutable_example` (bin "let_refutable_example") due to 1 previous error
```

这里有2种办法解决:

1. 在右值后边加上`else`语句,处理`None`变体

    ```rust
    fn main() {
        let a: Option<i32> = Some(5);
        // 使用else表达式时 要求该表达式的返回值类型和函数或闭包的返回值类型一致
        let Some(x) = a else { () };    // 这里如果走到else分支 则直接返回一个空的元组
        println!("{}", x);
    }
    ```
    
    ```
    cargo run
       Compiling let_refutable_else_example v0.1.0 (/let_refutable_else_example)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
         Running `target/debug/let_refutable_else_example`
    5
    ```
   
2. 使用`if let`语句

    ```rust
    fn main() {
        let a: Option<i32> = Some(5);
        if let Some(x) = a {
            println!("{}", x);
        }
    }
    ```
    
    ```
    cargo run
       Compiling if_let_refutable_example v0.1.0 (/if_let_refutable_example)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
         Running `target/debug/if_let_refutable_example`
    5
    ```
   
但如果`if let`语句后边跟了一个不可辩驳的模式,那么编译器会发出警告:

```rust
fn main() {
    // if let后跟一个不可辩驳的模式,则编译器会发出警告
    // 因为模式不会失败 所以if let是多余的
    if let x = 5 {      // warning: irrefutable if let pattern
        println!("x: {}", x);
    }
}
```

```
cargo run
   Compiling if_let_irrefutable_example v0.1.0 (/if_let_irrefutable_example)
warning: irrefutable `if let` pattern
 --> src/main.rs:2:8
  |
2 |     if let x = 5 {
  |        ^^^^^^^^^
  |
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`
  = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: `if_let_irrefutable_example` (bin "if_let_irrefutable_example") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/if_let_irrefutable_example`
x: 5
```

结合这个例子,可以想想一下`match`表达式:

- 最后一个分支应该是一个不可辩驳的模式
- 其他的分支应该是一个可辨驳的模式
- 因为`match`表达式要求所有的分支都要能够匹配到,所以最后一个分支必须是不可辩驳的模式