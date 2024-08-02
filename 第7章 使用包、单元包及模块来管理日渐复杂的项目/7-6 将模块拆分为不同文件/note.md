# PART1. 将模块内容移动到其他文件

模块定义时,如果模块名后是`;`而非`{}`:

- rust会从和模块同名的文件中加载内容(在同一目录下)
- 模块树的结构不变

例:

原文件如下:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- step1. 将`front_of_house`模块的内容移动到`front_of_house.rs`文件中

```
tree ./
./
├── Cargo.toml
└── src
    ├── front_of_house.rs
    └── lib.rs

1 directory, 3 files
```

`lib.rs`:

```rust
mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`front_of_house.rs`:

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

- step2. 将`hosting`模块的内容移动到`hosting.rs`文件中

这时再直接创建`hosting.rs`文件就不对了.需要先创建`src/front_of_house`目录,再在该目录下创建`hosting.rs`

也就是说,文件结构和模块树结构要保持一致.

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── front_of_house
    │         └── hosting.rs
    ├── front_of_house.rs
    └── lib.rs

2 directories, 5 files
```

- `front_of_house/hosting.rs`:

```rust
pub fn add_to_waitlist() {}
```

- `front_of_house.rs`:

```rust
pub mod hosting;
```

注:如果在`src`下同时存在`src/front_of_house.rs`和`src/front_of_house`目录,则编译器优先查找并加载`src/front_of_house/mod.rs`

这个`mod.rs`文件通常包含模块的内容,而不是直接包含代码.

等效修改:

```
tree ./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── front_of_house
    │         ├── hosting.rs
    │         └── mod.rs
    └── lib.rs

2 directories, 5 files
```

- `front_of_house/mod.rs`:

```rust
pub mod hosting;
```

- `front_of_house/hosting.rs`:

```rust
pub fn add_to_waitlist() {}
```

- `lib.rs`:

```rust
mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

# PART2. 我理解的一个工程结构

[参考工程](https://github.com/tokio-rs/mini-redis/tree/master/src)

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs                  // 用于声明根模块的子模块
    ├── main.rs                 // 项目入口
    └── my_module
        ├── mod.rs              // 用于声明子模块的内容 和 子模块的子模块
        └── sub_module.rs       // 子模块的子模块的内容

2 directories, 6 files
```
