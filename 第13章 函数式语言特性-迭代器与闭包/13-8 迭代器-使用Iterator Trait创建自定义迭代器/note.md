# PART1. 使用Iterator Trait创建自定义迭代器

其实核心思想就是实现Iterator Trait的`next()`方法,然后在`next()`方法中返回`Option<T>`,其中`T`是迭代器的元素类型

```rust
struct Counter {
    // 本字段用于存储迭代过程中的数值(也可以认为是状态)
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 指定关联类型为u32 这个语法后边再讲
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
```

```
cargo test
...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/iterator_example-0fd2a3466e444847)

running 1 test
test tests::calling_next_directly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests iterator_example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

为了使用一些其他的方法,现在创造了一个新的需求:

- 让2个Counter迭代器中的每个元素进行相乘,并将结果作为新的迭代器的元素
  - 第1个迭代器的元素为从1到5
  - 第2个迭代器的元素为从2到5
  - 期望的迭代器中的元素为(1*2, 2*3, 3*4, 4*5)
    - 注: 第2个迭代器中由于只有4个元素,因此假定最终结果中的第5个元素为None
- 对产生的新的迭代器中的元素有如下要求:
  - 要求其中的元素必须能够被3整除
  - 求符合这些要求的元素之和

- `skip()`方法: 跳过前n个元素,返回一个新的迭代器
- `zip()`方法: 将两个迭代器的元素进行配对,并返回一个元素类型为元组的新迭代器

```rust
struct Counter {
    // 本字段用于存储迭代过程中的数值(也可以认为是状态)
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 指定关联类型为u32 这个语法后边再讲
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let c1_iterator = Counter::new();
        let c2_iterator = Counter::new().skip(1);
        let c3_iterator = c1_iterator.zip(c2_iterator);

        let multi_closure = |a, b| a * b;
        let c4_iterator = c3_iterator.map(|(a, b)| multi_closure(a, b));

        let filter_closure = |x| x %3 == 0;
        let result_iterator = c4_iterator.filter(|x| filter_closure(*x));
        let result: u32 = result_iterator.sum();
        assert_eq!(result, 18);
    }

    #[test]
    // 上面的代码可以简化为下面的代码
    fn using_other_iterator_trait_methods1() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}
```