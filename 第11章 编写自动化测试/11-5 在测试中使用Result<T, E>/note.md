# PART1. 在测试中使用`Result<T,E>`

到目前为止,我们的测试只能在发生panic时失败.但是,有时我们想要测试一个函数返回的`Result<T,E>`的值,而不是它是否panic.这时,可以直接让测试函数返回`Result<T,E>`,而不是`()`.

- 测试函数返回`OK(())`表示测试通过
- 测试函数返回`Err`表示测试失败

```rust
#[cfg(test)]
mod tests {
    #[test]
    // 此处让测试函数返回Result即可
    fn two_plus() ->Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

```
cargo test
   Compiling result_in_test_fn_example v0.1.0 (/result_in_test_fn_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/result_in_test_fn_example-55a748ac139ffc9d)

running 1 test
test tests::two_plus ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests result_in_test_fn_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

需要注意的是,如果你想返回一个其他类型的`Ok`变体,那么需要确保这个类型实现了Termination trait.否则,编译器会报错.

若我们人为引入一个Bug,导致返回Err变体,则测试会失败:

```rust
#[cfg(test)]
mod tests {
    #[test]
    // 此处让测试函数返回Result即可
    fn two_plus() ->Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

```
cargo test
   Compiling result_in_test_fn_example v0.1.0 (/result_in_test_fn_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (target/debug/deps/result_in_test_fn_example-55a748ac139ffc9d)

running 1 test
test tests::two_plus ... FAILED

failures:

---- tests::two_plus stdout ----
Error: "two plus two does not equal four"


failures:
    tests::two_plus

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

注意:**不要在返回`Result<T,E>`的测试函数上标注`#[should_panic]`属性**