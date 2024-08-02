# PART1. 消耗迭代器的方法

- 在标准库中,Iterator Trait有一些带默认实现的方法
- 其中有一些方法会调用`next()`方法
  - 这也是实现Iterator Trait时必须实现`next()`方法的原因之一
- 调用`next()`的方法称为"消耗型迭代器"
  - 因为调用这些方法会把迭代器消耗尽
  - 例如:`sum()`方法
    - 该方法会取得迭代器的所有权
    - 通过反复调用`next()`方法,遍历所有元素
    - 每次迭代,都会把元素的值加到一个累加器上,迭代结束,返回累加器的值

例:

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // sum()方法会取得迭代器的所有权
    let total: i32 = v1_iter.sum();
    println!("{}", total);

    // 因此这里如果再次使用v1_iter会报错
    // println!("{:#?}", v1_iter);     // error[E0382]: borrow of moved value: `v1_iter`
}
```

# PART2. 产生其他迭代器的方法

定义在Iterator Trait上的另外一些方法叫做"迭代器适配器".这些方法可以把迭代器转换为不同种类的迭代器.

可以通过链式调用使用多个迭代器适配器来执行复杂的操作,这种调用可读性较高

例如:`map()`方法

- 该方法接收一个闭包作为参数,该闭包会被应用到迭代器的每个元素上
- 该方法返回一个新的迭代器,该迭代器会调用闭包来处理每个元素
- 由于该方法作用于一个迭代器上,并基于该迭代器创建一个新的迭代器,因此该方法被称为"map"

例:

```rust
fn main() {
  let v1 = vec![1, 2, 3];

  let add_one_closure = |x| {x + 1};

  // collect()方法可以将迭代器转换为集合
  // _表示让编译器推断元素的类型
  let v2: Vec<_> = v1.iter().map(add_one_closure).collect();
  println!("{:?}", v2);
}

```

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/map_example`
[2, 3, 4]
```

- `collect()`方法: 将迭代器转换为集合