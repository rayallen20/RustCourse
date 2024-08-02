# PART1. 按名称运行测试的子集

选择运行的测试:将测试的名称(1个或多个)作为`cargo test`命令的参数

## 1.1 运行单个测试

在`cargo test`命令后边指定要运行的测试名称即可

例:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(add_two(3), 5);
    }
    
    #[test]
    fn one_hundred() {
        assert_eq!(add_two(100), 102);
    }
}
```

运行全部测试:

```
cargo test
   Compiling example v0.1.0 (/example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.38s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

指定运行某1个测试:

```
cargo test add_three_and_two
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::add_three_and_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

## 1.2 运行多个测试

传入2个测试名称是不行的.想要运行多个测试,需要指定测试名的一部分,或者指定测试的模块名

例:运行以add开头的测试

```
cargo test add              
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```