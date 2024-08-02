# PART1. 函数指针

- 除了把闭包传递给函数之外,还可以把函数传递给函数
- 函数在传递的过程中会被强制转换为`fn`类型
- `fn`类型即为"函数指针"(Function Pointer)

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
```

```
cargo run
   Compiling fn_type_example_1 v0.1.0 (/fn_type_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.39s
     Running `target/debug/fn_type_example_1`
The answer is: 12
```

## 1.1 函数指针和闭包的区别

- `fn`是一个类型,而不是一个trait
  - 可以直接指定`fn`为参数类型,而不用声明一个以Fn Trait为约束的泛型参数
- 函数指针实现了全部3种闭包Trait(Fn、FnMut、FnOnce):
  - 总是可以把函数指针用作参数传递给一个接收闭包的函数
  - 所以,倾向于搭配实现了闭包Trait的泛型来编写函数,这样该函数就可以同时接收闭包和普通函数了
- 而某些情境下,只想接收`fn`类型,而不想接收闭包:
  - 与外部不支持闭包的代码交互:C函数

例1:

```rust
fn main() {
    println!("Hello, world!");
}

fn to_string_1(v: Vec<i32>) -> Vec<String> {
    v.iter()
        // 此处使用闭包 x的类型为&i32 返回值类型为String
        .map(|x| x.to_string())
        .collect()
}

fn to_string_2(v: Vec<i32>) -> Vec<String> {
    v.iter()
        // 此处使用函数指针 ToString是一个trait 该trait定义了to_string()方法
        // 所以此处传递的是一个函数指针
        .map(ToString::to_string)
        .collect()
}
```

再来看一下`map()`方法的定义:

```rust
pub trait Iterator {
    type Item;
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
        Self: Sized,
    {
        Map { it: self, f }
    }
}
```

该方法要求泛型`F`实现了`FnMut` Trait,而函数指针`ToString::to_string`和例子中的闭包都实现了该Trait

例2:

```rust
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    // Status::Value(3) 看起来和函数调用有些相似
    // 而实际上这种构造器确实被实现为了一个函数
    // 该函数接收一个参数并返回一个 Status::Value 变体
    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> = (0u32..20).
        // 所以可以把构造器作为实现了闭包Trait的函数指针来使用
        map(Status::Value).
        collect();

    println!("{:?}", list_of_statuses);
}
```

```
cargo run
...

warning: `fn_type_example_3` (bin "fn_type_example_3") generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/fn_type_example_3`

[Value(0), Value(1), Value(2), Value(3), Value(4), Value(5), Value(6), Value(7), Value(8), Value(9), Value(10), Value(11), Value(12), Value(13), Value(14), Value(15), Value(16), Value(17), Value(18), Value(19)]
```

# PART2. 返回闭包

- 闭包使用Trait进行表达,因此无法在函数中直接返回一个闭包Trait,但是可以将一个实现了该Trait的具型作为返回值

例:

```rust
fn return_closure() -> Fn(i32) -> i32 {     // error: doesn't have a size known at compile-time
  |x| x + 1
}

fn main() {
}
```

```
cargo check
    Checking return_closure_example_1 v0.1.0 (/return_closure_example_1)
error[E0746]: return type cannot have an unboxed trait object
 --> src/main.rs:1:24
  |
1 | fn return_closure() -> Fn(i32) -> i32 {
  |                        ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
help: box the return type, and wrap all of the returned values in `Box::new`
  |
1 ~ fn return_closure() -> Box<Fn(i32) -> i32> {
2 ~     Box::new(|x| x + 1)
  |

error[E0782]: trait objects must include the `dyn` keyword
 --> src/main.rs:1:24
  |
1 | fn return_closure() -> Fn(i32) -> i32 {
  |                        ^^^^^^^^^^^^^^
  |
help: use `impl Fn(i32) -> i32` to return an opaque type, as long as you return a single underlying type
  |
1 | fn return_closure() -> impl Fn(i32) -> i32 {
  |                        ++++
help: alternatively, you can return an owned trait object
  |
1 | fn return_closure() -> Box<dyn Fn(i32) -> i32> {
  |                        +++++++               +

Some errors have detailed explanations: E0746, E0782.
For more information about an error, try `rustc --explain E0746`.
error: could not compile `return_closure_example_1` (bin "return_closure_example_1") due to 2 previous errors
```

`doesn't have a size known at compile-time`: 该函数返回值的类型在编译时没有一个已知的大小

解决办法1: 使用`Box`包装返回值

```rust
type Closure = Box<dyn Fn(i32) -> i32>;

fn return_closure() -> Closure {
    Box::new(|x| x + 1)
}

fn main() {
    let f: Closure = return_closure();
    println!("{}", f(1));
}
```

```
cargo run
   Compiling return_closure_example_2 v0.1.0 (/return_closure_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/return_closure_example_2`
2
```

解决办法2: 返回函数指针

```rust
type Closure = fn(i32) -> i32;

fn return_closure() -> Closure {
    |x| x + 1
}

fn main() {
    let f: Closure = return_closure();
    println!("{}", f(1));
}
```

```
cargo run
   Compiling return_closure_example_3 v0.1.0 (/return_closure_example_3)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/return_closure_example_3`
2
```