# PART1. 控制测试如何运行

`cargo test`命令和`cargo run`命令类似.`cargo test`命令会在测试模式下编译代码并生成一个用于运行测试的可执行文件.

改变`cargo test`命令的行为: 添加命令行参数

`cargo test`命令的默认行为:

- 并行运行所有测试
- 在测试通过的前提条件下,捕获(即不显示)所有输出,使得读取测试结果更容易

命令行参数:

- 针对`cargo test`这个命令的参数: 这类参数紧跟在`cargo test`命令后边
- 针对`cargo test`这个命令生成的可执行文件的参数: 放在`-- `(注意这里还有个空格)后边

例:

- `cargo test --help`: 列出针对`cargo test`命令的参数
- `cargo test -- --help`: 列出针对`cargo test`命令生成的可执行文件的参数

```
cargo test -- --help
   Compiling result_in_test_fn_example v0.1.0 (/result_in_test_fn_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.39s
     Running unittests src/lib.rs (target/debug/deps/result_in_test_fn_example-55a748ac139ffc9d)
Usage: /result_in_test_fn_example/target/debug/deps/result_in_test_fn_example-55a748ac139ffc9d [OPTIONS] [FILTERS...]

Options:
        --include-ignored 
                        Run ignored and not ignored tests
        --ignored       Run only ignored tests
        --force-run-in-process 
                        Forces tests to run in-process when panic=abort
        --exclude-should-panic 
                        Excludes tests marked as should_panic
....
```

可以看到,在使用`cargo test -- --help`命令时,是有一个编译的过程的

# PART2. 并行运行测试和连续执行测试

## 2.1 并行运行测试

默认使用多个线程并行运行测试

但是这对测试函数有要求:

- 需要确保测试函数之间不会相互依赖
- 需要确保测试函数不依赖于某个共享状态(环境、工作目录、环境变量等)

## 2.2 连续执行测试

`--test-threads`参数: 指定并行运行测试的线程数

- 该参数是传递给二进制文件的
- 通常在不想以并行方式运行测试时使用该参数,或者需要对线程数进行细粒度控制时使用该参数
- 使用该参数后边跟着线程的数量即可指定线程数
  - 例如:`cargo test -- --test-threads=1` 表示只使用1个线程运行测试

# PART3. 显式函数输出

默认情况下,若测试通过,则Rust的test库会捕获所有打印到标准输出中的内容

例如:若被测试的代码中使用了`println!`,则在测试通过时,就不会在终端看到`println!`打印的内容

例:

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

```
cargo test -- --test-threads=1
   Compiling print_in_test_example v0.1.0 (/print_in_test_example)
warning: function `prints_and_returns_10` is never used
 --> src/lib.rs:1:4
  |
1 | fn prints_and_returns_10(a: i32) -> i32 {
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `print_in_test_example` (lib) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.69s
     Running unittests src/lib.rs (target/debug/deps/print_in_test_example-517088d7a92253b6)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

注意测试失败的信息:

```
failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

可以看到,仅在测试失败时才将被测试代码中`println!`的内容打印到终端

`--show-output`参数:该参数用于在测试通过的情况下,显示被测试函数中的标准输出

- 该参数是针对二进制文件的参数

例:

```
cargo test -- --test-threads=1 --show-output
warning: function `prints_and_returns_10` is never used
 --> src/lib.rs:1:4
  |
1 | fn prints_and_returns_10(a: i32) -> i32 {
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `print_in_test_example` (lib) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/print_in_test_example-517088d7a92253b6)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

```
successes:

---- tests::this_test_will_pass stdout ----
I got the value 4
```

可以看到,测试通过的情况下,函数的输出也显示了