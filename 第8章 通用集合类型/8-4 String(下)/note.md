# PART1. 对String按索引的形式进行访问

按索引语法访问String的某部分会报错

```rust
fn main() {
    let s = String::from("hello");
    let c = s[0]; // error: cannot index into a value of type `String`
}
```

```bash
cargo run
   Compiling index_string v0.1.0 (/index_string)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:15
  |
3 |     let c = s[0]; // error: cannot index into a value of type `String`
  |               ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`, which is required by `String: Index<_>`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  = help: the trait `SliceIndex<[_]>` is implemented for `usize`
  = help: for that trait implementation, expected `[_]`, found `str`
  = note: required for `String` to implement `Index<{integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `index_string` (bin "index_string") due to 1 previous error
```

Rust中,String不支持索引访问

# PART2. 内部表示

String是一个Vec<u8>的封装,每个字符占用1个字节

```rust
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), lang = "String")]
pub struct String {
    vec: Vec<u8>,
}
```

## 2.1 `len()`方法

String的`len()`方法返回的是字节数,而不是字符数

```rust
fn main() {
    // 1个英文字母在UTF-8编码下占1个字节
    let s = String::from("hello");
    let len_s = s.len();
    println!("The length of '{}' is {}.", s, len_s);

    // 1个汉字在UTF-8编码下占3个字节
    let s2 = String::from("你好");
    let len_s2 = s2.len();
    println!("The length of '{}' is {}.", s2, len_s2);

    // 在UTF-8编码中,我们将 "你" 这种字符称为Unicode标量值
    // 一个Unicode标量值可能由多个字节组成
    // 本例中的 "你" 字 由3个字节组成:
    // E4: 1110 0100 -> 0xE4 -> 228
    // BD: 1011 1101 -> 0xBD -> 189
    // A0: 1010 0000 -> 0xA0 -> 160
    // 字符串是字节的集合(Vec<u8>),如果允许索引访问,那么在本例中,访问
    // s2[0] 时,我们得到的是第一个字节的值,即 228
    // 这是一个很大的问题,因为我们期望得到的是字符 "你" 的Unicode标量值,而非某个字节的值
    // 因此,Rust不允许我们使用索引访问字符串
}
```

```bash
cargo run
   Compiling len_string v0.1.0 (/len_string)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.55s
     Running `target/debug/len_string`
The length of 'hello' is 5.
The length of '你好' is 6.
```

# PART3. 字节(Byte)、标量值(Scalar Value)、字形簇(Grapheme Clusters)

Rust有3种看待字符串的方式:

- 字节
- 标量值
- 字形簇(最接近"字母"的概念)

## 3.1 字节(Byte)

```rust
fn main() {
    let s = String::from("你好");

    // 以字节为单位遍历字符串
    // bytes()方法返回值是一个迭代器 可用于遍历字符串的字节
    for b in s.bytes() {
        println!("{}", b);
    }
}
```

```bash
 cargo run
   Compiling byte_string v0.1.0 (/byte_string)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/byte_string`
228
189
160
229
165
189
```

## 3.2 标量值(Scalar Value)

```rust
fn main() {
    let s = String::from("你好");

    // chars() 方法返回一个迭代器，迭代器的元素是字符串的 Unicode 标量值
    for c in s.chars() {
        println!("{}", c);
    }
}
```

```bash
cargo run
   Compiling unicode_scalar_value_string v0.1.0 (/unicode_scalar_value_string)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/unicode_scalar_value_string`
你
好
```

TODO: 这里如果使用印度语,你会发现虽然字符串中有4个字符,但是通过`chars()`方法遍历时,能得到5个字符,最后一个字符是音标,单独存在没有意义

## 3.3 字形簇(Grapheme Clusters)

这里需要引入一个crate: unicode-segmentation

使用这个库对字符串进行遍历,就能得到每一个字符了,和我们直觉上的字符是一致的

Rust不允许对String进行索引操作的最后一个原因:

- 索引操作应该消耗一个常量时间(O(1))
- 而String无法保证这一点:必须先遍历所有内容,才能确定到底有多少个合法的字符

# PART4. 切割String

可以使用`[]`和一个范围来创建字符串的切片

- 必须谨慎使用
- 如果切割时跨越了字符边界,会导致panic

```rust
fn main() {
    let s = String::from("你好");

    let s1 = &s[0..3];
    println!("{}", s1);
}
```

```bash
cargo run
   Compiling spilt_string v0.1.0 (/spilt_string)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/spilt_string`
你
```

如果我们不按照字符边界来切割字符串,则会发生panic:

```rust
fn main() {
    let s = String::from("你好");

    let s1 = &s[0..2];
    println!("{}", s1);
}
```

```bash
cargo run
   Compiling spilt_string v0.1.0 (/spilt_string)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/spilt_string`
thread 'main' panicked at src/main.rs:4:16:
byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`: 2不是一个字符边界,它在'你'这个字符的内部

# PART5. 遍历String

- 对于标量值: 使用`chars()`方法
- 对于字节: 使用`bytes()`方法
- 对于字形簇: 比较复杂,标准库没有提供.可以使用unicode-segmentation库

# PART6. 总结

Rust选择将正确处理String数据作为所有Rust程序的默认行为:程序员必须在处理UTF8数据之前投入更多精力

这样可以防止在开发后期涉及到非ASCII字符(简单理解就是非英文字符)时出现问题