# PART1. 测试(函数)

在Rust中,1个测试就是一个函数.这个函数用于验证非测试代码的功能是否和预期的一致

在一个测试函数体中,通常会执行3个操作:

- 准备数据/状态(Arrange)
- 运行被测试的代码(Act)
- 断言结果(Assert)

因此,这3个操作通常也被称为3A模式

# PART2. 剖析测试函数

- 测试函数需要使用`#[test]`属性(Attribute)标记
  - Attribute就是一段Rust代码的元数据,用于描述代码的特性
  - 在函数上边一行加上`#[test]`标注,即可把函数变为测试函数

# PART3. 运行测试

使用`cargo test`命令运行所有测试函数

- 该命令执行后,Rust会构建一个Test Runner可执行文件,并运行该文件
  - Test Runner会逐个运行标注了`#[test]`属性的测试函数,并报告其是否成功

使用`cargo new --lib`命令创建一个库项目时,会自动生成一个test module,其中有一个测试函数:

- 可以添加任意数量的test module或测试函数

```
 cargo new adder --lib
    Creating library `adder` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

```
tree ./adder 
./adder
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files
```

其中`lib.rs`文件中有一个测试函数:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

函数`it_works()`是一个测试函数.再次强调,它之所以是一个测试函数,是因为它被`#[test]`属性标记了,而非是因为该函数所属的模块名为`tests`

运行测试:

```
cargo test
   Compiling adder v0.1.0 (/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.32s
     Running unittests src/lib.rs (target/debug/deps/adder-2c449bbcb2038bd9)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `running 1 test`: 表示运行了1个测试函数
- `test tests::it_works`: 表示运行的测试为`tests`模块下的`it_works()`函数
  - `... ok`: 表示测试通过
- `test result: ok.`: 表示这个项目中所有的测试都通过了
  - `1 passed`: 表示1个测试通过
  - `0 failed`: 表示0个测试失败
  - `0 ignored`: 表示0个测试被忽略(测试函数是可以被忽略的)
  - `0 measured`: 表示0个性能测试
  - `0 filtered out`: 表示0个测试被过滤
- `Doc-tests adder`: 表示运行了0个文档测试
  - Rust能够编译出现在API文档中的代码.这一特性用于保证文档始终和代码是同步的

## 3.1 修改函数`it_works()`的名称

将函数`it_works()`的函数名修改为`exploration()`,并运行测试:

`src/lib.rs`:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

```
cargo test
   Compiling adder v0.1.0 (/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running unittests src/lib.rs (target/debug/deps/adder-2c449bbcb2038bd9)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `test tests::exploration`: 表示运行的测试为`tests`模块下的`exploration()`函数

# PART4. 测试失败

- 测试函数中一旦触发了panic就意味着测试失败了
- 每个测试在运行时是处于一个独立的线程中的,因此一个测试的panic不会影响其他测试的运行
- 主线程负责监视测试线程的运行,当主线程监视到某个测试线程panic时,则该测试会被标记为失败

`src/lib.rs`:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

```
cargo test
   Compiling adder v0.1.0 (/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running unittests src/lib.rs (target/debug/deps/adder-2c449bbcb2038bd9)

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at src/lib.rs:17:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

- `running 2 tests`:运行了2个测试
  - `test tests::exploration ... ok`:exploration测试通过了
  - `test tests::another ... FAILED`:another测试失败了
    - `thread 'tests::another' panicked at src/lib.rs:17:9:`:测试失败的原因为在该处发生了panic

```
failures:
    tests::another
    ...
```

此处列出了所有失败的测试

- `test result: FAILED`:总体测试结果为失败
  - `1 passed`:1个测试通过
  - `1 failed`:1个测试失败
  - `0 ignored`:0个测试被忽略
  - `0 measured`:0个性能测试
  - `0 filtered out`:0个测试被过滤