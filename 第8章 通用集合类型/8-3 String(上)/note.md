# PART1. 字符串是什么

- Byte的集合
- 提供了一些方法,这些方法能够将byte解析为文本

- Rust的核心语言层面,只有一个字符串类型:字符串切片`str`(或`&str`)
- 字符串切片: 对存储在只读内存中的、UTF-8编码的字符串的引用
  - 字符串字面量: 是字符串切片(`let s = "hello";`)

- String类型
- 来自标准库,而非核心语言
- 可增长、可变、拥有所有权
- UTF-8编码

通常所说的字符串,指的就是String类型和&str类型.这两种类型在标准库中用的都比较多,而且都是UTF-8编码的

# PART2. 其他类型的字符串

Rust标准库还包含了很多其他的字符串类型,如:OsString、OsStr、CString、CStr等

通常,以String结尾的类型是拥有所有权的;以Str结尾的类型是引用类型

这些类型可存储不同编码的文本,或者在内存中以不同的形式展现

某些library crate针对存储字符串提供了更多的类型

# PART3. 创建一个新的字符串(String)

因为String本质是Byte的集合,所以很多对Vec<T>的操作也适用于String

使用`String::new()`创建一个空的字符串

```rust
fn main() {
    let s = String::new();
}
```

使用初始值创建String

`to_string()`方法(注意`to_string()`是方法)可用于实现了Display trait的类型,将字面量转换为String

```rust
fn main() {
    let data: &str = "initial value";
    let data_string: String = data.to_string();
    
    let data_string2: String = "initial value".to_string();
}
```

`String::from()`函数也可用于从字面量创建String

```rust
fn main() {
    let s = String::from("hello");
}
```

以上两种方式是等效的.但是由于在Rust中,字符串用的地方非常多,所以提供了多种创建字符串的方式

# PART4. 更新String

## 4.1 `push_str()`方法

`push_str()`方法将一个字符串切片附加到String

```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
```

注意`push_str()`方法的签名:

```rust
pub fn push_str(&mut self, string: &str)
```

它接收的是一个借用,因此它不会获得所有权

```rust
fn main() {
    let mut s = String::from("hello");
    let s2: String = String::from(", world!");
    s.push_str(&s2);
    // 此处打印s2是不会报错的 因为push_str()方法不会获取s2的所有权,而是获取了s2的引用
    println!("{}", s2);
}
```

```bash
cargo run
   Compiling push_str v0.1.0 (/push_str)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.04s
     Running `target/debug/push_str`
, world!
```

## 4.2 `push()`方法

`push()`方法将一个字符附加到String

```rust
fn main() {
    let mut s = String::from("hello");
    s.push('l');
}
```

## 4.3 拼接字符串

### 4.3.1 使用`+`运算符

`+`运算符实际上是调用了一个类似如下签名的方法:

```rust
fn add(self, other: &str) -> String
```

这里之所以说是类似,是因为Rust中的`add()`方法使用了泛型,这里我们只是使用了&str这个具型来表达它

`add()`方法是会取得实例的所有权的,所以在使用`+`运算符时,会将实例的所有权转移给`+`运算符,而`+`运算符会返回一个新的String实例

```rust
fn main() {
  let s1 = String::from("hello, ");
  let s2 = String::from("world!");

  // + 运算符调用了 add 方法
  // add 方法的签名是 fn add(self, s: &str) -> String
  let s3 = s1 + &s2;
  println!("{}", s3);
  println!("{}", s2); // 从方法签名可以看出,s2 的所有权没有被转移,只是发生了借用, 因此 s2 仍然可以使用

  // 而s1的所有权已经被转移,在这里s1已经失效了,所以这里会报错
  println!("{}", s1); // error: value borrowed here after move
}
```

另外还有一个问题:`add()`方法的参数类型为`&str`,而s2是String类型,为什么能够编译通过呢?

这是因为Rust提供了一个叫做`Deref`的trait,这个trait称为解引用强制转换(deref coercion),它允许Rust自动将`&String`类型转换为`&str`类型

### 4.3.2 使用`format!`宏

`format!`宏类似于`println!`宏,但是它并不会打印出来,而是返回一个String实例(类似Sprintf)

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s4);

    // 等效于使用 format! 宏
    let s1 = String::from("tic");
    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s5);
}
```

```bash
cargo run
   Compiling format v0.1.0 (/format)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/format`
tic-tac-toe
tic-tac-toe
```

需要注意的是,`format!`宏并不会获取任何参数的所有权,所以这里的`s1`、`s2`、`s3`仍然可以使用