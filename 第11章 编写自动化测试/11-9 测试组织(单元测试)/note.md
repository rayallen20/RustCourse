# PART1. 测试的分类

Rust对测试的分类:

- 单元测试
- 集成测试

单元测试:

- 小、专注
- 1次对1个模块进行隔离的测试
- 可以测试私有的接口

集成测试:

- 在库外部.和其他外部代码一样使用你的代码
- 只能测试公有的接口
- 可以在测试中使用多个模块

# PART2. 单元测试

单元测试的目的在于将一小段代码隔离开,从而迅速地确定这段代码是否符合预期.

且一般把单元测试放在与被测试的代码同级的目录下,并且在同一个文件中.

约定俗成的,每个源代码文件,都要建立一个同名的测试文件,并且在文件中使用`#[cfg(test)]`标注测试代码.

tests模块上的`#[cfg(test)]`标注:

- 只有在运行`cargo test`命令才编译和运行的代码
- 运行`cargo build`时则不会

而集成测试则是在不同的目录中,且它不需要使用`#[cfg(test)]`标注.

- cfg: configuration(配置)
  - 被该属性标注的代码,只有在特定的配置下才会被编译
  - 配置选项`test`表示只有在运行测试时才编译这部分代码,这个选项是由Rust提供的
  - 因此,只有在执行`cargo test`命令时,才会编译和运行被`#[cfg(test)]`标注的代码.包括模块中的helper函数和`#[test]`标注的函数

例:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
    
    // 这个函数也会在测试时被编译
    fn nothing() {
        
    }
}
```

# PART3. 测试私有函数

Rust的测试是可以访问私有函数的.

例:

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// 私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 调用父模块的私有函数
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

```
cargo test
   Compiling test_private_fn_example v0.1.0 (/test_private_fn_example)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests src/lib.rs (target/debug/deps/test_private_fn_example-8237b91ff44ab7e1)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests test_private_fn_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```