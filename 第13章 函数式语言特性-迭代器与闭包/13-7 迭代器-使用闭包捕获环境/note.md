# PART1. 使用闭包捕获环境

`filter()`方法:

- 该方法接收一个闭包
- 该闭包在遍历迭代器中的每个元素时,返回一个bool类型
- 如果闭包返回True,则当前元素将会包含在`filter()`方法返回的迭代器中
- 如果闭包返回False,则当前元素将不会包含在`filter()`方法返回的迭代器中

`lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        let filter_closure = |s: &Shoe| s.size == size;

        // into_iter()方法会获取集合的所有权
        shoes.into_iter().filter(filter_closure).collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }
}
```

```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/filter_example-8eb68621e224e7de)

running 1 test
test tests::filter_by_size ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/filter_example-4ee6d44ea92cd400)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests filter_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```