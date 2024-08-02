本章讲解的所有数据类型,均为存储在Heap上的数据

# PART1. Vector

`Vec<T>`:

- 由标准库提供
- 可存储多个值
- 只能存储相同类型的数据
- 值在内存中连续存放

## 1.1 创建Vector

```rust
fn main() {
    // 使用new()函数创建Vector
    let v: Vec<i32> = Vec::new();

    // 使用宏创建Vector
    let v2: Vec<i32> = vec![1, 2, 3];
}
```

这里Vector是一种泛型类型,所以需要指定类型参数`<i32>`(泛型后边再讲)

## 1.2 更新Vector

### 1.2.1 向Vector中添加元素

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}
```

### 1.2.2 删除Vector

和其他struct一样,当Vector离开作用域后:

- Vector自身会被drop
- Vector中的元素也会被drop

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
} // v会在离开作用域时被drop;同时v中的元素也会被drop;但是如果涉及到v中的元素被引用时,会怎么样?
```

## 1.3 访问Vector中的元素

- 索引
- `get()`方法

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引访问
    let third: &i32 = &v[2];

    // get()方法返回一个Option<&T>类型
    match v.get(2) {
        // Some: 存在指定索引的元素
        Some(element) => println!("The third element is {}", element),
        // None: 不存在指定索引的元素
        None => println!("There is no third element."),
    }
}
```

```bash
cargo run
   Compiling visit_vector v0.1.0 (/visit_vector)
warning: unused variable: `third`
 --> src/main.rs:5:9
  |
5 |     let third: &i32 = &v[2];
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_third`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `visit_vector` (bin "visit_vector") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/visit_vector`
The third element is 3
```

二者在处理越界时的行为不同:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引访问 越界会导致panic
    let third: &i32 = &v[100];
}
```

```bash
cargo run
   Compiling index_overshoot_vector_index v0.1.0 (/index_overshoot_vector_index)
warning: unused variable: `third`
 --> src/main.rs:5:9
  |
5 |     let third: &i32 = &v[100];
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_third`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `index_overshoot_vector_index` (bin "index_overshoot_vector_index") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/index_overshoot_vector_index`
thread 'main' panicked at src/main.rs:5:25:
index out of bounds: the len is 5 but the index is 100
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```
thread 'main' panicked at src/main.rs:5:25:
index out of bounds: the len is 5 but the index is 100
```

使用索引访问时,如果索引越界,会导致panic

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match v.get(100) {
        Some(element) => println!("Item 100 is {}", element),
        None => println!("Sorry, this vector is too short.")
    }
}
```

```bash
 cargo run
   Compiling index_overshoot_vector_get v0.1.0 (/index_overshoot_vector_get)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/index_overshoot_vector_get`
Sorry, this vector is too short.
```

使用`get()`方法访问时,如果索引越界,会返回`None`(枚举Option的变体)

### 1.3.1 索引和`get()`面对越界时的区别

- 索引: 越界会导致panic
- `get()`: 越界会返回`None`

## 1.4 所有权和借用规则

不能在同一作用域内同时存在可变和不可变引用

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 此处是对v的一个不可变引用

    // 此处是对v的可变引用
    v.push(6); // error: cannot borrow `v` as mutable because it is also borrowed as immutable

    // 此处是对v的不可变引用
    println!("The first element is: {}", first);

    // 这里之所以对vector尾部做修改,而不允许引用vector头部的原因在于:
    // vector在内存中是连续存储的,如果在vector尾部添加元素,可能会导致vector重新分配内存,从而导致原来的引用失效
    // 而本例中的 first 仍然指向原来的内存地址,所以不允许对vector同时存在可变和不可变引用
}
```

```bash
cargo run
   Compiling borrow_vector v0.1.0 (/borrow_vector)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
3 |     let first = &v[0]; // 此处是对v的一个不可变引用
  |                  - immutable borrow occurs here
...
6 |     v.push(6); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
  |     ^^^^^^^^^ mutable borrow occurs here
...
9 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `borrow_vector` (bin "borrow_vector") due to 1 previous error
```

## 1.5 遍历Vector

### 1.5.1 仅遍历Vector中的元素

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }
}
```

```bash
cargo run
   Compiling traverse_vector v0.1.0 (/traverse_vector)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/traverse_vector`
1
2
3
4
5
```

### 1.5.2 遍历Vector中的元素并修改

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        // *: 解引用符 用于取出引用的值
        *i += 50;
    }

    println!("{:?}", v);
}
```

```bash
cargo run
   Compiling traverse_and_change_vector v0.1.0 (/traverse_and_change_vector)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/traverse_and_change_vector`
[51, 52, 53, 54, 55]
```