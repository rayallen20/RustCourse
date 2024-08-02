# PART1. Cargo工作空间(Workspaces)

cargo工作空间:帮助管理多个相互关联且需要协同开发的crate

cargo工作空间实际上就是一套共享同一个`Cargo.lock`和输出文件夹的,包含多个crate的项目

# PART2. 创建工作空间

有多种方式来创建工作空间

例:创建一个工作空间,该工作空间中:

- 包含1个二进制crate
- 包含2个库crate
- 二进制crate依赖于其他2个库crate
- 其中1个库crate提供`add_one()`函数
- 另1个库crate提供`add_two()`函数

- step1. 创建工作空间目录和`Cargo.toml`文件

```
~ % mkdir add
~ % cd add 
add % touch Cargo.toml
```

该`Cargo.toml`文件的用途是配置整个工作空间,而不是单个crate

- step2. 编辑`add/Cargo.toml`

由于该`Cargo.toml`文件是工作空间的配置文件,所以其中既不包含`package`区域,也不包含元数据

```toml
[workspace]

# 为工作区添加成员
# 此处添加的成员是相对路径 即成员相对于Cargo.toml文件的路径
members = [
    "adder",
]
```

- step3. 创建二进制crate

`adder` crate是一个二进制crate,它依赖于`add_one`和`add_two`库crate

```
add % cargo new adder
    Creating binary (application) `adder` package
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

```
tree ./
./
├── Cargo.toml
└── adder
    ├── Cargo.toml
    └── src
        └── main.rs

2 directories, 3 files
```

- step4. 构建工作空间

```
add % cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling adder v0.1.0 (/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.23s
```

```
add % tree ./ -L 3 -P '*|target/*' --prune
./
├── Cargo.lock
├── Cargo.toml
├── adder
│         ├── Cargo.toml
│         └── src
│             └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── adder
        └── adder.d

4 directories, 7 files
```

可以看到,编译产出物全都放在`add/target`目录下.相应的,`adder` crate就没有自己的`target`目录了.

且`Cargo.lock`文件也是共享的.

即使在`add/adder`目录下进行编译,也不会生成`target`目录

```
add % cd adder 
adder % cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
```

```
adder % tree ./
./
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

之所以将`target`目录设置为共享,是因为往往同一个工作空间下的crate之间会有相互依赖关系,如果每个crate都有自己的`target`目录,那么它们在各自编译时,就不得不再次编译自身依赖的crate,这样会浪费时间和空间.

因此,将`target`目录设置为共享,可以避免重复编译.

- step5. 创建库crate

```toml
[workspace]

# 为工作区添加成员
# 此处添加的成员是相对路径 即成员相对于Cargo.toml文件的路径
members = [
    "adder",
    "add_one",
]
```

```
add % cargo new add_one --lib
    Creating library `add_one` package
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`add/add_one/src/lib.rs`:

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

- step6. 让`adder` crate依赖于`add_one` crate

`add/adder/Cargo.toml`:

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]

add-one = { path = "../add_one"}
```

工作空间内的carte不会假设彼此依赖,所以需要显式声明依赖关系.

`src/main.rs`:

```rust
use add_one::add_one;

fn main() {
    let num = 10;
    println!("{} plus one is {}", num, add_one(num));
}
```

```
add % cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling add_one v0.1.0 (/add/add_one)
   Compiling adder v0.1.0 (/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
```

```
cargo run -p adder
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/adder`
10 plus one is 11
```

`-p`参数:指定要运行的crate的名称

# PART3. 在工作空间中依赖外部crate

工作空间中只有1个`Cargo.lock`文件,在工作空间的顶层目录

- 这个做法保证了工作空间内所有的crate使用的依赖的版本都是相同的
- 这也意味着工作空间内的所有crate相互兼容,因为它们依赖的版本都是相同的

例: 为不同的crate添加不同版本的依赖,并查看`Cargo.lock`文件

- step1. 为`add_one` crate添加依赖

`add/add_one/Cargo.toml`:

```toml
[package]
name = "add_one"
version = "0.1.0"
edition = "2021"

[dependencies]

rand = "0.3.14"
```

- step2. 为`adder` crate添加一个不同版本的依赖

`add/adder/Cargo.toml`:

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]

add_one = { path = "../add_one"}

rand = "0.3.15"
```

- step3. 构建工作空间

```
add % cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling libc v0.2.155
   Compiling rand v0.4.6
   Compiling rand v0.3.23
   Compiling add_one v0.1.0 (/add/add_one)
   Compiling adder v0.1.0 (/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.04s
```

- step4. 查看`Cargo.lock`

```toml
[[package]]
name = "rand"
version = "0.3.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "64ac302d8f83c0c1974bf758f6b041c6c8ada916fbb44a609158ca8b064cc76c"
dependencies = [
    "libc",
    "rand 0.4.6",
]
```

此处可以看到,虽然2个crate分别指定了不同版本的`rand` crate,但是最后实际使用的版本是`0.3.23`

这一点在`Cargo.lock`中也可以证实:

```toml
[[package]]
name = "add_one"
version = "0.1.0"
dependencies = [
 "rand 0.3.23",
]

[[package]]
name = "adder"
version = "0.1.0"
dependencies = [
 "add_one",
 "rand 0.3.23",
]
```

注意:工作空间内的其他crate,若其`Cargo.toml`文件中没有指定使用`rand`依赖,则无法使用`rand` crate

# PART4. 为工作空间添加测试

`add/add_one/src/lib.rs`:

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

```
add % cargo test
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling add_one v0.1.0 (/add/add_one)
   Compiling adder v0.1.0 (/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/debug/deps/add_one-e45384fa0094bfcc)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-31e5802a0f5b33a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

一块一块的分析:

```
     Running unittests src/lib.rs (target/debug/deps/add_one-e45384fa0094bfcc)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`add_one`中有一个测试,并且测试通过

```
Running unittests src/main.rs (target/debug/deps/adder-31e5802a0f5b33a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`adder`中没有测试

这一例子证明,在工作空间中执行`cargo test`命令,会一次性执行所有crate中的测试

**注意:在发布crate时,无法将工作空间内的所有crate一次性发布.必须切换到每一个crate内执行`cargo publish`命令**