# PART1. 添加自定义错误信息

可以向`assert!`、`assert_eq!`、`assert_ne!`添加可选的自定义信息

- 这些自定义信息和失败信息都会被打印
- `assert!`:第1个参数必填,自定义消息作为第2个参数
- `assert_eq!`和`assert_ne!`:第1个参数和第2个参数必填,自定义消息作为第3个参数
- 自定义信息参数会被传递给`format!`宏,因此在自定义信息中可以使用`{}`占位符

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_contain_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

```
cargo test
   Compiling custom_info_example v0.1.0 (/custom_info_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.63s
     Running unittests src/lib.rs (target/debug/deps/custom_info_example-1342b6cfc2228667)

running 1 test
test tests::greetings_contain_name ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests custom_info_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

现在引入一个Bug:

```rust
pub fn greeting(name: &str) -> String {
    // 引入Bug
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_contain_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

此时测试失败:

```
cargo test
   Compiling custom_info_example v0.1.0 (/custom_info_example)
warning: unused variable: `name`
 --> src/lib.rs:1:17
  |
1 | pub fn greeting(name: &str) -> String {
  |                 ^^^^ help: if this is intentional, prefix it with an underscore: `_name`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `custom_info_example` (lib) generated 1 warning
warning: `custom_info_example` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.70s
     Running unittests src/lib.rs (target/debug/deps/custom_info_example-1342b6cfc2228667)

running 1 test
test tests::greetings_contain_name ... FAILED

failures:

---- tests::greetings_contain_name stdout ----
thread 'tests::greetings_contain_name' panicked at src/lib.rs:13:9:
assertion failed: result.contains("Carol")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greetings_contain_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

此时测试失败的信息为:

```
thread 'tests::greetings_contain_name' panicked at src/lib.rs:13:9:
assertion failed: result.contains("Carol")
```

此时只能从失败信息中看出`result`不包含`"Carol"`,无法看出`result`的值是什么,这时可以添加自定义信息:

```rust
pub fn greeting(name: &str) -> String {
    // 引入Bug
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_contain_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

```
cargo test
   Compiling custom_info_example v0.1.0 (/custom_info_example)
warning: unused variable: `name`
 --> src/lib.rs:1:17
  |
1 | pub fn greeting(name: &str) -> String {
  |                 ^^^^ help: if this is intentional, prefix it with an underscore: `_name`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `custom_info_example` (lib) generated 1 warning
warning: `custom_info_example` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests src/lib.rs (target/debug/deps/custom_info_example-1342b6cfc2228667)

running 1 test
test tests::greetings_contain_name ... FAILED

failures:

---- tests::greetings_contain_name stdout ----
thread 'tests::greetings_contain_name' panicked at src/lib.rs:13:9:
Greeting did not contain name, value was `Hello!`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greetings_contain_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

此时测试失败的信息为:

```
thread 'tests::greetings_contain_name' panicked at src/lib.rs:13:9:
Greeting did not contain name, value was `Hello!`
```