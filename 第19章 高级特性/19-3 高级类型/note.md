# PART1. 使用new type模式实现类型安全和抽象

上一节中已经使用过new type模式了,这里就讲一下它的用途:

- 用来静态的保证各种值之间不会混淆,并表明值的单位
- 为类型的某些细节提供抽象能力
- 通过轻量级的封装来隐藏内部实现细节
  - 这一点有点像设计模式中的外观模式(门面模式),但是new type模式更加简单,只是简单的封装了一下

# PART2. 使用类型别名创建类型同义词

Rust提供了类型别名的功能:

- 为现有类型产生另外的名称(同义词)
- 类型的别名并不是一个独立的类型
- 使用`type`关键字来定义类型别名
- 不可以在类型的别名上实现方法

类型别名的主要用途:减少代码的重复

例1:

```rust
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    // 类型别名可以与原类型交互 因为类型别名仅仅只是一个别名 不是一个新的类型
    println!("x + y = {}", x + y);
}
```

```
cargo run
   Compiling type_alias_example_1 v0.1.0 (/type_alias_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.96s
     Running `target/debug/type_alias_example_1`
x + y = 15
```

例2:

```rust
fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    Box::new(|| println!("hi"))
}

fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
}
```

在这段代码中,`Box<dyn Fn() + Send + 'static>`被重复输入了多次,可以使用类型别名来简化:

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
}
```

例3:

`std::io`包中的`Result`类型就是一个类型别名:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub type Result<T> = result::Result<T, Error>;
```

其中`Error`变体是`std::io::Error`类型:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Error {
    repr: Repr,
}
```

而泛型`T`则表示IO操作的结果类型

在IO操作中,`std::io::Error`类型可以表示绝大多数的IO错误,因此`std::io`包中定义了`std::io::Result`类型别名,用于表示IO操作的结果

现有代码如下:

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

fn main() {
}
```

简化的思路和标准库的代码也是相同的:

```rust
use std::fmt;

// 为Result<T, std::io::Error>枚举定义别名
pub type MyResult<T> = Result<T, std::io::Error>;

pub trait Write {
  fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
  fn flush(&mut self) -> MyResult<()>;

  fn write_all(&mut self, buf: &[u8]) -> MyResult<()>;
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> MyResult<()>;
}

fn main() {
}
```

# PART3. never类型

有一种名为`!`的特殊类型:

- 它没有任何值,我们称为空类型(empty type)
- 我们倾向于叫它`never`类型,因为它在不返回的函数中充当返回类型
- 不返回值的函数也被称为发散函数(diverging function)

例1:

```rust
// 返回!表示该函数永远不会返回值
fn bar() -> ! {
    // 该函数无返回值
    // 但这并不代表什么都不返回,Rust中无返回值的函数默认返回单元类型 (即空元组 () )
    // 也就是说函数实际上还是返回了值的
}   // error: type mismatch: expected `!`, found `()`

fn main() {}
```

例2:

```rust
fn main() {
    let guess = "";

    loop {
        // match的各个arm的返回值类型必项一致
        // Ok arm返回的是u32类型
        // 而Err arm返回的是continue
        // continue返回的就是一个 never 类型 (!类型)
        // 但never类型无法产生一个可供返回的值
        // 因此这个match表达式的值就采用了Ok arm的返回值类型 即u32类型
        // 而never类型则被强制转换为了其他任意类型 (never类型的表达式可以被强制转换为任意其他类型)
        // 因此这2个arm返回的类型是一致的
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
```

注意:`never`类型可以被强制转换为任意其他类型

例3:

```rust
impl <T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            // Some arm返回的类型为T
            Some(val) => val,
            // None arm返回的类型为 panic!()宏的返回值类型 即never类型
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

例4:

```rust
fn main() {
    print!("forever ");
    
    // 这里的loop表达式返回的类型即为`!`
    loop {
        // 无限循环
        print!("and ever ");
    }
}
```

# PART5. 动态大小和Sized trait

Rust中的类型有两种形式:

- 动态大小类型(DST,Dynamic Sized Type)
  - 无法在编译时确定大小的类型,只能在运行时才能确定大小
  - 例如`str`类型,它的大小是在运行时才能确定的
  - 以下代码无法正常工作:
    - `let s1: str = "hello";`
    - `let s2: str = "world!";`
    - `s1`和`s2`均为`str`类型,但是它们的大小是不同的,而同一类型的所有值必须占有等量的内存,因此这段代码无法通过编译
  - 解决办法:使用`&str`类型
    - `&str`是一个指向`str`类型的引用,该类型存储的是`str`类型的地址和长度,因此其大小是固定的
- 静态大小类型(SST,Static Sized Type)

## 5.1 Rust使用动态大小类型的通用方式

附带一些额外的元数据来存储动态信息的大小:

- 使用动态大小的类型时,总会把它的值放在某种指针后边
- 例如`str`和`&str`

## 5.2 另外一种动态大小的类型: trait

- 每个trait都是一种动态大小的类型,可以通过名称对其进行引用
- 为了将trait用作trait对象,必须将它放置在某种指针之后,例如:
  - `&dyn Trait`或`Box<dyn Trait>`(`Rc<dyn Trait>`)

## 5.3 Sized Trait

- 为了处理动态大小的类型,Rust提供了一个Sized Trait,用于确定一个类型的大小在编译时是否已知
  - 编译时可计算出大小的类型会自动实现这一trait
  - Rust会为每一个泛型函数隐式的添加Sized约束

例:

```rust
fn generic<T> (t: T) {}

// 以上函数会被隐式转换为如下形式:
// fn generic<T: Sized> (t: T) {}

fn main() {}
```

默认情况下,泛型函数只能用于编译时已知大小的类型,可以通过特殊语法解除这一限制:

## 5.4 ?Sized Trait约束

```rust
// ?表示不确定性 表示泛型T可能是Sized也可能不是Sized
// 但是?只能用在Sized trait上,不能用于其他trait
// 另外,由于T的大小可能是不确定的,因此需要把类型T放在某种指针之后,比如&T或者Box<T>
fn generic<T: ?Sized> (t: &T) {
    // do something
}

fn main() {}
```

- `T`可能是Sized,也可能不是Sized
- 该语法只能用在Sized trait上,不能用于其他trait