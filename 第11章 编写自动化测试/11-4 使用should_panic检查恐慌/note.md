# PART1. 验证错误处理的情况

测试除了验证代码的返回值是否正确之外,还需要验证代码是否如预期的处理了发生错误的情况

比如需要编写一个测试用例,该用例用于验证代码在特定的情况下是否发生了panic

- `should_panic`属性(attribute):
  - 若测试函数中的代码发生了panic,则该测试用例会被标记为通过
  - 若测试函数中的代码没有发生panic,则该测试用例会被标记为失败

例:

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```
cargo test
   Compiling should_panic_example v0.1.0 (/should_panic_example)
warning: field `value` is never read
 --> src/lib.rs:2:5
  |
1 | pub struct Guess {
  |            ----- field in this struct
2 |     value: u32,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `should_panic_example` (lib) generated 1 warning (1 duplicate)
warning: `should_panic_example` (lib test) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running unittests src/lib.rs (target/debug/deps/should_panic_example-f7f02aef71715fa3)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests should_panic_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

测试结果:

```
running 1 test
test tests::greater_than_100 - should panic ... ok
```

可以看到,由于发生了panic,所以该函数顺利通过了测试

我们再次修改代码,引入一个Bug,取消掉大于100的边缘条件检测,然后再次运行测试:

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // if value < 1 || value > 100 {

        // 人为引入Bug
        if value < 1{
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```
cargo test
   Compiling should_panic_example v0.1.0 (/should_panic_example)
warning: field `value` is never read
 --> src/lib.rs:2:5
  |
1 | pub struct Guess {
  |            ----- field in this struct
2 |     value: u32,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `should_panic_example` (lib) generated 1 warning
warning: `should_panic_example` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/should_panic_example-f7f02aef71715fa3)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到,由于没有触发panic,因此该用例没有通过测试

但是我们观察测试失败的信息:

```
---- tests::greater_than_100 stdout ----
note: test did not panic as expected
```

它仅仅指出没有发生panic,但是并没有指出具体的错误原因,这样的信息对于我们来说是不够的.

假设被测试的代码中有多个panic点,即使触发了panic,我们也无法得知这个panic是否是我们预期的panic

# PART2. 让`should_panic`更加精确

为`should_panic`属性添加一个可选的`expected`参数:

- 该参数用于检查失败消息中是否包含所指定的文字

例:

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic (expected = "Guess value must be less than or equal to 100")] // panic中的信息包含expected参数中的内容即可通过测试
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```
cargo test
   Compiling expected_example v0.1.0 (/expected_example)
warning: field `value` is never read
 --> src/lib.rs:2:5
  |
1 | pub struct Guess {
  |            ----- field in this struct
2 |     value: u32,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `expected_example` (lib) generated 1 warning
warning: `expected_example` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/expected_example-0b9df73e0f1cdd2e)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests expected_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

若触发的panic信息中包含了`expected`参数中的内容,则该测试用例会被标记为通过;否则会被标记为失败:

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic (expected = "Guess value must be less than or equal to 100")] // panic中的信息包含expected参数中的内容即可通过测试
    fn greater_than_100() {
        Guess::new(0);
    }
}
```

```
 cargo test
   Compiling expected_example v0.1.0 (/expected_example)
warning: field `value` is never read
 --> src/lib.rs:2:5
  |
1 | pub struct Guess {
  |            ----- field in this struct
2 |     value: u32,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `expected_example` (lib) generated 1 warning
warning: `expected_example` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/lib.rs (target/debug/deps/expected_example-0b9df73e0f1cdd2e)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at src/lib.rs:8:13:
Guess value must be greater than or equal to 1, got 0.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 0."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到,由于触发的panic不包含`expected`参数中的内容,因此该测试用例没有通过测试:

```
thread 'tests::greater_than_100' panicked at src/lib.rs:8:13:
Guess value must be greater than or equal to 1, got 0.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 0."`,
 expected substring: `"Guess value must be less than or equal to 100"`
```