# PART1. 宏(macro)

宏在Rust里指的是一组相关特性的集合称谓:

- 使用`macro_rules!`构建声明宏(declarative macros)
- 3种过程宏:
  - 自定义`#[derive]`宏,用于struct或enum,可以为其指定随derive属性添加的代码
  - 类似属性的宏,可以在任何语法项上添加自定义属性
  - 类似函数的宏,看起来像函数调用,对其指定为参数的token流进行操作

# PART2. 函数和宏的区别

- 本质上,宏是用来编写可以生成其他代码的代码(元编程, metaprogramming)
- 函数在定义签名时,必须声明参数的个数和类型,而宏则可以接受任意数量的参数
- 编译器会在解释代码前展开宏,而函数则是在运行时执行
- 宏的定义比函数复杂的多,因为宏的参数可以是任意的Rust代码,而函数的参数必须是确定的类型.这导致宏的代码难以阅读、理解、测试
- 在某个文件调用宏时,必须提前定义宏或者将宏引入当前作用域
- 函数可以在任何位置定义和调用,而宏必须在调用之前定义

# PART3. `macro_rules!`声明宏(弃用)

声明宏又被称为宏模板、`macro_rules!`声明宏,有时候也被直接叫做宏

这种宏即将被弃用,因为它们的功能有限,而且不够灵活

- 声明宏是Rust中最常见的宏形式:
  - 它有些类似match表达式,但是声明宏匹配的是代码而不是值
  - 定义声明宏时,需要使用`macro_rules!`关键字

`lib.rs`:

```rust
// let v: Vec<u32> = vec![1, 2, 3];

// #[macro_export]: 表示该宏需要在其所属的包被引入作用域后,方可使用
// 没有该标注的宏,则不能被引入作用域
#[macro_export]
// 本代码为一个简化版的vec!宏的实现
// macro_rules!关键字: 用于定义宏
macro_rules! vec {
    // 声明宏内部的代码有点像match表达式的各个分支
    // 本例中,只有1个分支 即: ( $( $x:expr ), * ) 可以把它看成是一个模式
    // 由于我们自己手搓的vec宏只有1种有效的模式,因此其他模式会导致编译错误 可以认为如果我们调用这个自己手搓的vec宏时
    // 传入的参数不符合这个模式,那么编译器会报错
    // 某些比较复杂的宏,则会有多个模式
    // 但是声明宏中的代码和match表达式还是有本质区别的 声明宏匹配的是Rust的代码结构 match表达式匹配的是值

    // $x:expr: expr是一个模式,表示任意表达式;$x是一个变量名,表示匹配到的任意表达式
    // 这句话的意思就是匹配任何表达式,并将其绑定到变量$x上
    // ,: 表示被捕获的表达式后边可能(注意是可能而不是必须)会出现一个,
    // $( $x:expr )中的$(): 表示捕获的表达式将被包装成一个数组
    // *: 表示捕获的表达式可以出现0次或多次 这里捕获表达式指的就是 $( $x:expr )
    // 也就是说,这里捕获到的表达式 就是 let v: Vec<u32> = vec![1, 2, 3]; 中的 1 2 3
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            // 这里的 $()*表示针对 $( $x:expr ), * 捕获到的每一个表达式$x
            // 都执行一次其中的代码块
            // $()内部是要重复执行的代码
            // *表示这段代码可以被重复执行0次或多次 具体数量取决于有多少个$x被捕获到
            $(
                temp_vec.push($x);
            )*
            // 这里的temp_vec是一个表达式,表示这个宏最终会返回temp_vec
            temp_vec
        }
    };
}

// 这个宏最终生成的代码效果大致如下:
// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
// temp_vec
```

`main.rs`:

```rust
use declarative_macros_example::vec;
fn main() {
    let v = vec!(1, 2, 3);
    println!("{:?}", v);
}
```

```
cargo run
   Compiling declarative_macros_example v0.1.0 (/declarative_macros_example)
    Building [=============>               ] 1/2: declarative_macros_example(bin)                                                                                                                                                    
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.59s
     Running `target/debug/declarative_macros_example`
[1, 2, 3]
```

由于`macro_rules!`宏有一些奇怪的技术细节,因此Rust团队正在试图使用`macro!`宏来替代`macro_rules!`宏

# PART4. 过程宏

过程宏是一种更加通用的宏,可以接受任意类型的参数,并且可以返回任意类型的代码

- 过程宏更像函数(某种形式的过程)一些
  - 接收并操作输入的Rust代码
  - 生成另外一些Rust代码作为结果

- 过程宏有3种类型:
  - 自定义派生宏(自定义`#[derive]`宏) 
  - 属性宏
  - 函数宏

- 创建过程宏时:
  - 宏定义必须单独放在它们自己的包中,并使用特殊的包类型

```rust
use proc_macro::TokenStream;

// #[some_attribute]: 用于指定过程宏类型的占位符
// TokenStream: 用于表示输入和输出的 token 序列 可以简单理解为该类型表示过程宏输入和输出的Rust代码
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    input
}
```

## 4.1 自定义派生宏(自定义derive宏)

我们通过一个例子来介绍自定义派生宏.其需求如下:

- 创建一个名为`hello_macro`的lib crate,其中包含一个名为`HelloMacro`的Trait
  - 该Trait中定义了一个关联函数`hello_macro()`
- 我们需要提供一个能够自动实现该Trait的过程宏
- 用户在他们自定义的类型(struct或enum)上标注`#[derive(HelloMacro)]`属性时,会自动实现`HelloMacro`Trait

### 4.1.1 创建工作空间

```
mkdir hello_macro_example
cd hello_macro_example/
touch Cargo.toml
```

`hello_macro_example/Cargo.toml`:

```toml
[workspace]

members = [
  # 该lib crate用于定义HelloMacro trait
  "hello_macro",
  # 该lib crate用于定义HelloMacro trait的derive宏(宏的定义需要单独放在一个crate中)
  "hello_macro_derive",
  # 该 binary crate 用于测试hello_macro和hello_macro_derive
  "pancakes"
]
```

### 4.1.2 创建`hello_macro` lib crate

```
cargo new hello_macro --lib
    Creating library `hello_macro` package
warning: compiling this new package may not work due to invalid workspace configuration
...
```

`hello_macro_example/hello_macro/src/lib.rs`:

```rust
pub trait HelloMacro {
  fn hello_macro();
}
```

### 4.1.3 创建`pancakes` binary crate

```
cargo new pancakes
    Creating binary (application) `pancakes` package
warning: compiling this new package may not work due to invalid workspace configuration

...
```

`hello_macro_example/pancakes/Cargo.toml`:

```toml
[package]
name = "pancakes"
version = "0.1.0"
edition = "2021"

[dependencies]
hello_macro = { path = "../hello_macro" }
```

此处以`pancakes/src/main.rs`为例,演示我们想实现的效果:

```rust
use hello_macro::HelloMacro;

// 需求1: 为用户自定义的类型添加 #[derive(HelloMacro)] 宏
// 即可为该类型添加HelloMacro宏的默认实现
struct Pancakes;

impl HelloMacro for Pancakes {
  // 需求2: 在默认实现中,要求打印出 "Hello, Macro! My name is XXX!"
  // 其中XXX为用户自定义的类型名称
  fn hello_macro() {
    println!("Hello, Macro! My name is Pancakes!");
  }
}

fn main() {
  Pancakes::hello_macro();
}

// 大致实现完成的效果如下:
// #[derive!(HelloMacro)]
// struct Pancakes;
// fn main() {
//     Pancakes::hello_macro();    // Hello, Macro! My name is Pancakes!
// }
```

### 4.1.4 创建`hello_macro_derive` lib crate

过程宏需要放置在单独的包中,因此我们需要创建一个新的lib crate

```
cargo new hello_macro_derive --lib
    Creating library `hello_macro_derive` package
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`hello_macro_example/hello_macro_derive/Cargo.toml`:

```toml
[package]
name = "hello_macro_derive"
version = "0.1.0"
edition = "2021"

# lib配置段:用户指定该Rust包作为lib crate时的一些配置选项
# 通常用于指定库的名称、路径等
[lib]
# proc-macro选项: 用于指定该lib crate是一个过程宏
# 开启该选项时,该库将会被编译为一个过程宏库
proc-macro = true

[dependencies]
# 本包用于将Rust代码转化为可以进一步操作的数据结构(抽象语法树)
syn = "0.14.4"
# 本包用于将抽象语法树转化为Rust代码
quote = "0.6.3"
```

`hello_macro_example/hello_macro_derive/src/lib.rs`:

```rust
extern crate proc_macro;

// proc_macro包提供了编译器接口 其中的TokenStream类型用于表示Rust代码
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

// #[proc_macro_derive(HelloMacro)] 宏用于定义一个自定义的派生宏
// 简单理解就是 当你在一个类型上标注 #[derive(XXX)] 属性时
// Rust编译器会自动查找并调用 #[proc_macro_derive(XXX)] 指定的函数
#[proc_macro_derive(HelloMacro)]
/// 本函数将在类型添加了 #[derive(HelloMacro)] 属性时,被自动调用
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // 将输入的 TokenStream(可以理解为就是输入的Rust代码) 转换成AST
  let ast = syn::parse(input).unwrap();

  // 为输入的代码提供HelloMacro trait的实现
  impl_hello_macro(&ast)
}

/// 本函数用于为输入的代码提供HelloMacro trait的实现
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  // 获取添加了 #[derive(HelloMacro)] 属性的类型名称
  let name = &ast.ident;

  // 为该类型实现 HelloMacro trait并提供实现
  // quote! 宏: 用于将宏内部的Rust代码转换成TokenStream
  let gen = quote! {
        // 此处的 #name 是一个插值表达式,用于将name变量的值插入到代码中
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 宏: 用于将传入的表达式转换成字符串
                // 例如: stringify!(1 + 2) 将返回 "1 + 2"
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

  // quote! 宏返回的TokenStream是一种编译器无法直接理解的数据结构,
  // 因此需要将其转换成编译器可以理解的TokenStream
  gen.into()
}
```

编译:

```
cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
...
```

### 4.1.5 测试

`hello_macro_example/pancakes/src/Cargo.toml`:

```toml
[package]
name = "pancakes"
version = "0.1.0"
edition = "2021"

[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro_derive" }
```

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// 需求1: 为用户自定义的类型添加 #[derive(HelloMacro)] 宏
// 即可为该类型添加HelloMacro宏的默认实现
#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Apples;

// impl HelloMacro for Pancakes {
//     // 需求2: 在默认实现中,要求打印出 "Hello, Macro! My name is XXX!"
//     // 其中XXX为用户自定义的类型名称
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
  Pancakes::hello_macro();
  Apples::hello_macro();
}

// 大致实现完成的效果如下:
// #[derive!(HelloMacro)]
// struct Pancakes;
// fn main() {
//     Pancakes::hello_macro();    // Hello, Macro! My name is Pancakes!
// }
```

```
cargo run
...
Hello, Macro! My name is Pancakes
Hello, Macro! My name is Apples
```

## 4.2 属性宏

- 属性宏与自定义derive宏类似
  - 允许创建新的属性
  - 但不是为derive属性生成代码
- 属性宏更加灵活
  - derive宏只能用于struct和enum
  - 属性宏可以用于任何语法项,例如函数

例:

```rust
#[route(GET, "/")]
// 在web项目中,通常为handleFunc添加route属性
fn index() {}

#[proc_macro_attribute]
// 而实际上route这个宏就是通过 #[proc_macro_attribute] 来定义的
// 其中 attr参数是属性的内容(在本例中是GET, "/")
// item参数是属性所修饰的函数(在本例中是fn index() {})
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
}
```

除此之外,属性宏和derive宏的实现方式基本一致.也是需要定义一个开启了`proc-macro`选项的lib crate,并在其中实现属性宏

## 4.3 函数宏

- 函数宏的定义类似于函数调用,但比普通参数更加灵活
- 函数宏可以接收TokenStream作为参数
- 与另外两种过程宏类似,在定义中使用Rust代码来操作TokenStream

```rust
let sql = sql!("SELECT * FROM `user`");

#[proc_macro]
// 本宏用于解析 SQL 语句
// 这个宏的实现要比macro_rules!复杂得多 因为其中不仅要匹配到Rust代码的语法结构 还要匹配到SQL语句的语法结构
pub fn sql(input: TokenStream) -> TokenStream {

}
```