# PART1. 什么是迭代器

- 迭代器模式: 对一系列项执行某些任务
- 迭代器在该模式中的作用:
  - 遍历每个项
  - 确定遍历何时完成
- Rust的迭代器:
  - 惰性的:除非调用消费迭代器的方法,否则迭代器本身没有任何效果
  - 大概可以理解为:迭代器被创建后,当你不使用这个迭代器时,这个迭代器就相当于没有做任何操作;当你调用了某些可以消耗迭代器的方法时,迭代器才会开始工作

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    // v1_iter就是一个迭代器 但到这行代码为止 这个迭代器还没有被使用
    // 因此这个迭代器还没有任何效果
    let v1_iter = v1.iter();

    // 此处才开始使用迭代器
    for element in v1_iter {
        println!("Got: {}", element);
    }
}
```

# PART2. Iterator Trait

- 所有的迭代器都实现了Iterator Trait
- 定义大致如下:

```rust
pub trait Iterator {
    type Item;
  
    fn next(&mut self) -> Option<Self::Item>;
  
    // 其他方法省略
}
```

这里有2个新的语法: `type Item`和`Self::Item`.这两个语法定义了与该Trait关联的类型.这块后续再讲

现在你只需要知道:实现Iterator Trait需要你定义一个Item类型,该类型用于`next`方法的返回值类型(实际上这个类型就是迭代器返回的类型)

Iterator Trait仅要求实现1个方法:`next()`

`next()`方法:

- 每次调用该方法时,都返回迭代器中的一项
- 返回结果包裹在`Option`枚举的`Some`变体中
- 当迭代器结束时,返回`None`变体

实际使用时,可以直接在迭代器上调用`next()`方法:

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    // 这里加了mut关键字 是因为next方法会改变迭代器的内部状态
    // 可以理解为 next方法消耗掉了迭代器中的一个元素
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}
```

而在刚刚的`for`循环的例子中,是因为`for`循环取得了迭代器的所有权,并在循环内部已经将迭代器变为可变的了,因此不需要加`mut`关键字

```rust
fn main() {
  let v1 = vec![1, 2, 3];
  
  let v1_iter = v1.iter();
  
  // for循环会取得迭代器的所有权 并在循环内部将迭代器修改为可变的
  // 因此使用for循环遍历迭代器时 不需要加mut关键字
  for element in v1_iter {
    println!("Got: {}", element);
  }
}
```

# PART3. 几个迭代方法

- `iter()`方法: 基于元素的不可变引用创建迭代器
- `into_iter()`方法: 使集合的所有权转移给迭代器,迭代器会消耗集合并返回元素.
  - 该方法在迭代元素时,会将元素的所有权移动到新的作用域内,并取得元素的所有权
- `iter_mut()`方法: 基于元素的可变引用创建迭代器

