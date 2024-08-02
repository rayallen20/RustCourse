# PART1. 使用`assert!`宏检查测试结果

- `assert!`宏来自标准库,用于确定某个状态是否为true.该宏可以接收一个返回值类型为bool的表达式
  - 表达式的值为true:测试通过
  - 表达式的值为false:`assert!`宏会调用`panic!`宏,表示测试失败,并打印出表达式的值

例:

```
cargo new assert_example --lib
```

`src/lib.rs`:

```rust
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // tests模块只是一个普通的模块 在该模块中想要使用其他模块的内容
    // 同样需要先导入 此处导入的是lib.rs中的所有公有项
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // Arrange: 准备数据阶段
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        // Act: 执行代码阶段
        let result = larger.can_hold(&smaller);

        // Assert: 验证结果阶段
        assert!(result);
    }
}
```

```
cargo test
   Compiling assert_example v0.1.0 (/assert_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.12s
     Running unittests src/lib.rs (target/debug/deps/assert_example-531626856888daf5)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

再添加一个测试用例:

```rust
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // tests模块只是一个普通的模块 在该模块中想要使用其他模块的内容
    // 同样需要先导入 此处导入的是lib.rs中的所有公有项
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // Arrange: 准备数据阶段
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        // Act: 执行代码阶段
        let result = larger.can_hold(&smaller);

        // Assert: 验证结果阶段
        assert!(result);
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        let result = smaller.can_hold(&larger);

        // 此处result的值应该是false 加了取反 断言结果为true
        assert!(!result);
    }
}
```

```
cargo test
   Compiling assert_example v0.1.0 (/assert_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src/lib.rs (target/debug/deps/assert_example-531626856888daf5)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_can_not_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

接下来我们修改`can_hold()`方法,模拟一个错误的实现:

```rust
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height

        // 模拟一个错误的实现
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // tests模块只是一个普通的模块 在该模块中想要使用其他模块的内容
    // 同样需要先导入 此处导入的是lib.rs中的所有公有项
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // Arrange: 准备数据阶段
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        // Act: 执行代码阶段
        let result = larger.can_hold(&smaller);

        // Assert: 验证结果阶段
        assert!(result);
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        let result = smaller.can_hold(&larger);

        // 此处result的值应该是false 加了取反 断言结果为true
        assert!(!result);
    }
}
```

```
cargo test
   Compiling assert_example v0.1.0 (/assert_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/assert_example-531626856888daf5)

running 2 tests
test tests::smaller_can_not_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'tests::larger_can_hold_smaller' panicked at src/lib.rs:39:9:
assertion failed: result
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到此时有一个测试用例失败了,说明我们实现的逻辑是有问题的,这样就可以及时发现问题并进行修复.

# PART2. 使用`assert_eq!`和`assert_ne!`宏测试相等性

- `assert_eq!`和`assert_ne!`宏用于检查两个值是否相等或不相等
  - 这两个宏都来自标准库 
  - `assert_eq!`宏接收两个参数,如果两个参数相等,测试通过
  - `assert_ne!`宏接收两个参数,如果两个参数不相等,测试通过
  - 实际上它们就是在`assert!`宏的基础上使用`==`和`!=`操作符进行封装
  - 通常我们把被测试的结果作为一个参数,把期望的结果作为另一个参数传入
  - 若断言失败:自动打印出2个参数的值
    - 这两个宏对参数有一定的要求.参数必须实现了`PartialEq` trait和`Debug` trait.
      - 因为它们在打印时会调用`Debug`trait,所以要求参数实现了`Debug`trait(所有的基本类型和标准库中的大部分类型都实现了`Debug` trait和`PartialEq` trait)
      - 对于自定义的struct和enum,则需自行实现这2个trait

## 2.1 `assert_eq!`宏的例子

```
cargo new assert_eq_example --lib
    Creating library `assert_eq_example` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`src/lib.rs`:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // Rust中,期待的值和执行结果,放在具体哪个位置上都可以
        // 但在一些其他语言中,可能会对位置有要求
        assert_eq!(4, add_two(2));
    }
}
```

```
cargo test
   Compiling assert_eq_example v0.1.0 (/assert_eq_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src/lib.rs (target/debug/deps/assert_eq_example-a1e405ab6f213f9e)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_eq_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

引入一个逻辑错误:

`src/lib.rs`:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // Rust中,期待的值和执行结果,放在具体哪个位置上都可以
        // 但在一些其他语言中,可能会对位置有要求
        assert_eq!(4, add_two(2));
    }
}
```

此时测试会失败:

```
cargo test
   Compiling assert_eq_example v0.1.0 (/assert_eq_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src/lib.rs (target/debug/deps/assert_eq_example-a1e405ab6f213f9e)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at src/lib.rs:13:9:
assertion `left == right` failed
  left: 4
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
assertion `left == right` failed
  left: 4
 right: 5
```

这部分即为`assert_eq!`宏的输出信息,可以看到左边的值是4,右边的值是5,说明测试失败了.

## 2.2 `assert_ne!`宏的例子

```
cargo new assert_ne_example --lib
    Creating library `assert_ne_example` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`src/lib.rs`:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_ne!(5, add_two(2));
    }
}
```

```
cargo test
   Compiling assert_ne_example v0.1.0 (/assert_ne_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/lib.rs (target/debug/deps/assert_ne_example-9ad2e44742159432)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_ne_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```