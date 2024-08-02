# PART1. 集成测试

在Rust中,集成测试完全位于被测试库的外部.这意味着集成测试不可以访问被测试库的私有函数,而只能调用被测试库的公有函数.

集成测试的目的:验证被测试库的多个部分是否能正确的一起工作.

因此,集成测试的覆盖率很重要

# PART2. tests目录

Rust的集成测试文件位于tests目录中.该目录和src目录同级.

tests目录下的每个测试文件都是单独的一个crate

- tests目录下的集成测试文件,需要将被测试的库导入
- tests目录下的测试函数无需标注`#[cfg(test)]`,因为该目录下的文件会被特殊对待
  - 只有执行`cargo test`命令,才会编译tests目录下的文件

现有一项目如下:

```
tree ./
./
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files
```

`src/lib.rs`:

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

- step1. 手动创建tests目录

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests

2 directories, 3 files
```

- step2. 在tests目录中手动创建测试文件

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_tests.rs

2 directories, 4 files
```

`tests/integration_tests.rs`:

```rust
use example;

#[test]
fn it_adds_two() {
    assert_eq!(4, example::add_two(2));
}
```

```
cargo test
   Compiling example v0.1.0 (/example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.59s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_tests.rs (target/debug/deps/integration_tests-fea5a72be58ca9d6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

单元测试区域:

```
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

集成测试区域:

```
     Running tests/integration_tests.rs (target/debug/deps/integration_tests-fea5a72be58ca9d6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- 每个集成测试文件都会有一个单独的集成测试区域
  - 因此通常执行集成测试时,都是有选择的执行
- `test it_adds_two ... ok`:集成测试中的每一个测试函数都会占1行

# PART3. 运行指定的集成测试

- 运行一个特定的集成测试: `cargo test 函数名`
- 运行某个测试文件内的所有测试: `cargo test --test 文件名`

例:

- step1. 创建一个新的集成测试文件`tests/another_integration.rs`:

```rust
use example;

#[test]
fn it_really_adds_two() {
    assert_eq!(5, example::add_two(3));
}
```

- step2. 只运行`integration_tests.rs`中的测试函数

```
cargo test --test integration_tests
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running tests/integration_tests.rs (target/debug/deps/integration_tests-fea5a72be58ca9d6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# PART4. 集成测试中的子模块

tests目录下的每个文件都被编译为一个单独的crate,而这些文件不共享行为,这一点和src目录下的文件不同.

但是如果在集成测试中,也需要共享一些行为,类似src中建立一些子模块一样,该怎么办呢?

首先我们尝试在tests目录下直接定义一个共享的函数:

- step1. 在tests目录下创建一个共享的函数文件`common.rs`:

```rust
// 本函数用于模拟一个集成测试中的公共函数
pub fn setup() {
    
}
```

此时执行`cargo test`命令:

```
cargo test
   Compiling example v0.1.0 (/example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.85s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/another_integration.rs (target/debug/deps/another_integration-4d12988d47886d36)

running 1 test
test it_really_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-a1846f51588ea589)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_tests.rs (target/debug/deps/integration_tests-fea5a72be58ca9d6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到:

```
     Running tests/common.rs (target/debug/deps/common-a1846f51588ea589)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

很明显,`common.rs`文件被当做一个单独的测试文件来对待了,而我们想要的是该文件中的代码作为一个共享的函数而被其他文件使用.

解决方案:在tests目录下建立一个子模块即可

- step1. 在tests目录下创建一个子模块目录`common`:

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── another_integration.rs
    ├── common    // 新建的子模块目录
    ├── common.rs
    └── integration_tests.rs

3 directories, 6 files
```

- step2. 在`common.rs`中定义`mod.rs`:

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── another_integration.rs
    ├── common
    │   └── mod.rs
    ├── common.rs
    └── integration_tests.rs

3 directories, 7 files
```

`tests/common/mod.rs`:

```rust
pub fn setup() {

}
```

- step3. 删除`tests/common.rs`

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── another_integration.rs
    ├── common
    │   └── mod.rs
    └── integration_tests.rs

3 directories, 6 files
```

- step4. 在`tests/integration_tests.rs` 和 `tests/another_integration.rs`中导入`common`模块并使用即可:

`tests/integration_tests.rs`:

```rust
use example;

// 使用集成测试中的子模块
mod common;

#[test]
fn it_adds_two() {
    // 使用集成测试中的子模块中的函数
    common::setup();
    assert_eq!(4, example::add_two(2));
}
```

`tests/another_integration.rs`:

```rust
use example;

mod common;

#[test]
fn it_really_adds_two() {
    common::setup();
    assert_eq!(5, example::add_two(3));
}
```

tests目录下的子目录,不会被视为单独的crate,而是被视为一个子模块,因此可以在不同的集成测试文件中共享.且不会被当做单独的测试文件来对待.

```
cargo test
   Compiling example v0.1.0 (/example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.40s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/another_integration.rs (target/debug/deps/another_integration-4d12988d47886d36)

running 1 test
test it_really_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_tests.rs (target/debug/deps/integration_tests-fea5a72be58ca9d6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到,此时`common`子模块不再被当做单独的测试文件来对待了.

# PART5. 针对binary crate的集成测试

- 如果项目是binary crate,则只含有`src/main.rs`而没有`src/lib.rs`
  - 此时就不能在tests目录下创建集成测试了
  - 退一步讲,就算有测试,也不能把`src/main.rs`导入到测试包的作用域中
- 因为只有library crate才能暴露函数给其他crate适用
- 而binary crate意味着要独立运行

所以通常针对binary crate的项目,通常都把逻辑放在`lib.rs`中,`main.rs`中只保留一些胶水代码.

这样在集成测试时,就可以把这个binary crate的项目视为一个library crate了.这样就可以通过use导入到测试包的作用域中了.