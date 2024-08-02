# PART1. 测试驱动开发

- step1. 编写一个会失败的测试,运行该测试,确保它是按预期的原因失败
- step2. 编写或修改刚好足够的代码,让测试通过
- step3. 重构代码,在重构的同时确保测试能始终通过
- step4. 返回step1,继续

# PART2. 使用TDD编写搜索功能

## 2.1 编写测试用例

`src/lib.rs`中添加测试用例:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
```

## 2.2 编写代码

很明显此时我们还没有实现`search()`函数.这一步为了让编译通过,我们先将`search()`函数声明出来:

`src/lib.rs`:

```rust
// 此处由于vector中的元素是从content中获取的 所以vector中的元素和content的生命周期是一样的
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    vec![]
}
```

## 2.3 运行测试

当然此时我们的测试肯定无法通过,因为我们还没有实现`search()`函数:

```
cargo test
   Compiling minigrep v0.1.0 (/minigrep)
...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.24s
     Running unittests src/lib.rs (target/debug/deps/minigrep-46eec06a0a17d646)

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at src/lib.rs:48:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

很明显,`assert_eq!`的左值和右值是不等的.

## 2.4 实现`search()`函数

- `str.lines()`方法: 返回一个迭代器,迭代器的每个元素是`str`的一行

`src/lib.rs`:

```rust
// 此处由于vector中的元素是从content中获取的 所以vector中的元素和content的生命周期是一样的
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}
```

再次运行测试:

```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep-46eec06a0a17d646)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-5ff5a5b93151e998)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

此时测试可以顺利通过了.

顺利通过测试,表明`search()`函数的实现是正确的,因此可以在`run()`函数中调用`search()`函数了:

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content :String = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}
```

测试:

```
cargo run frog poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep frog poem.txt`
Search for frog
In file poem.txt
How public, like a frog
```