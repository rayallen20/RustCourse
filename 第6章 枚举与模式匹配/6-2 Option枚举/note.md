# PART1. Option枚举

Option枚举定义于标准库中,在Prelude中预导入.它描述了如下情况:

某个值可能存在(这个值是某种类型),也可能不存在的情况

# PART2. Rust中没有Null

在其他语言中,我们常常使用Null来表示一个值不存在的情况.一个变量可以处于2种状态:

- 空值(null)
- 非空

这种设计有一个很大的问题:当你尝试像使用非Null值一样使用Null值时,就会引发某种错误(比如NPE,Null Pointer Exception)

Null的设计者称它为Billions Dollar Mistake(百亿美元的错误).

但是Null的概念还是有用的:它描述了因某种原因而变为无效或缺失的值

# PART3. Rust中类似Null概念的枚举-Option<T>

Option<T>在Rust中的定义:

```rust
/// The `Option` type. See [the module level documentation](self) for more.
#[derive(Copy, PartialOrd, Eq, Ord, Debug, Hash)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow(clippy::derived_hash_with_manual_eq)] // PartialEq is specialized
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

`Some`变体可以关联一个类型`T`(其中`T`为泛型参数),表示这个值存在,而`None`变体表示这个值不存在

例:

```rust
fn main() {
    // some_number的类型为 Option<i32> 而非i32
    let some_number = Some(5);

    // some_string的类型为 Option<&str> 而非&str
    let some_string = Some("a string");

    // 无法通过None变体来推断出absent_number的类型 因此需要显式指定类型
    let absent_number: Option<i32> = None;
}
```

# PART4. Option<T>比Null好在哪?

在Rust中,Option<T>和T是不同的类型,不可以把Option<T>直接当成T使用

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let sum = x + y;
}
```

```bash
cargo run
   Compiling option_t_not_t v0.1.0 (/option_t_not_t)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <i8 as Add>
            <i8 as Add<&i8>>
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `option_t_not_t` (bin "option_t_not_t") due to 1 previous error
```

error[E0277]: cannot add `Option<i8>` to `i8`:不能把`Option<i8>`和`i8`相加

也就是说,想要使用Option<T>中的T,则必须先将其类型转换为T.这就避免了"假定一个值是非Null的,但其实它是Null"这种错误