# PART1. 忽略某些测试,运行剩余测试

- `ignore`属性: 被该属性标记的测试函数将在执行`cargo test`时被忽略

例:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(5, 1 + 1 + 1 + 1 + 1)
    }
}
```

```
cargo test
   Compiling example v0.1.0 (/example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.29s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到,`expensive_test`测试函数被忽略了

运行被标记为`ignore`的测试函数:

`--ignored`:运行被忽略的测试

- 该参数针对二进制文件使用

例:

```
cargo test -- --ignored
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/example-fea0d0589eaca54f)

running 1 test
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```