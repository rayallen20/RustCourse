# PART1. 使用`pub use`重新导出名称

使用`use`将路径导入到作用域后,该名称在此作用域内是私有的,不能被外部访问.但是可以使用`pub use`重新导出名称,使其可以被外部访问.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 此时外部的代码是无法访问hosting模块的,因为仅使用use关键字
// 导入的模块在本作用域内是私有的
// use front_of_house::front_of_house;

// 如果想要让外部的代码也能访问到hosting模块,就要使用pub use
// 相当于外部代码认为hosting模块是在当前作用域下定义的
// 这样的方式使得代码实际存在的位置和外部代码看到的位置不一致
// 进而导致代码的可读性下降,因为外部代码无法直观的看到hosting模块是在哪个作用域下定义的
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`pub use`的作用:

- 将条目引入作用域
- 该条目可以被外部代码引入到它们的作用域内

# PART2. 使用外部包(package)

- step1. 在`Cargo.toml`文件中添加依赖
  - 这里会从`crates.io`上下载对应的包
- step2. 使用`use`关键字导入包

例:

- step1. 添加依赖

```toml
[package]
name = "use_package"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.5.5"
```

- step2. 使用`use`导入包

```rust
use rand::Rng;

fn main() {
    
}
```

在rust中,标准库也被当做外部包来使用,所以可以使用`use`导入标准库中的模块.但是不需要修改`Cargo.toml`文件,因为标准库是默认存在的.

例:

```rust
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
```

# PART3. 使用嵌套路径清理大量的use语句

如果使用同一个包或模块下的多个条目,可以使用嵌套路径的方式来简化代码.

`路径相同的部分::{路径差异的部分}`

例:

```rust
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");
}
```

可以改写为:

```rust
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");
}
```

## 3.1 一个引用是另一个引用的子路径

例:既要使用`std::io`又要使用`std::io::Write`

```rust
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
}
```

可以改写为:

```rust
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
}
```

这里的`self`即为`std::io`

# PART4. 通配符`*`

作用:使用`*`可以导入一个模块下的所有公有条目

```rust
use std::collections::*;

fn main() {
    println!("Hello, world!");
}
```

注意:谨慎使用通配符,因为这样会导入大量的名称,可能会导致命名冲突

应用场景:

- 测试:将所有被测试的代码引入到tests模块中
- 有时被用于预导入(预导入,即prelude.是指将所有的公有条目导入到一个模块中,然后在其他模块中使用这个模块)模块
