# PART1. 不安全Rust

目前为止学到的Rust,在编译期都强制检查了内存安全,但是Rust也提供了一种不安全的操作,这种操作是为了解决一些特殊的问题,比如和C语言交互,或者是在性能要求非常高的场景下.

- Rust中隐藏着第二语言,它没有强制内存安全保证:Unsafe Rust(不安全的Rust)
  - 和普通Rust一样,但提供了额外的"超能力"

- Unsafe Rust存在的原因:
  - 静态分析是保守的
    - 编译器在判断一段代码是否安全时,会拒绝一些实际上是安全的代码,只是编译器认为这些代码可能不安全
    - 使用Unsafe Rust: 相当于告诉编译器,我知道自己在做什么,并承担相应风险
  - 计算机硬件本身就是不安全的,Rust需要能够进行底层系统编程

# PART2. Unsafe的超能力

- 使用`unsafe`关键字可以切换到不安全模式,开启一个代码块,该代码块中放着一些不安全的操作
  - 但是`unsafe`并不是一个魔法的开关,只是告诉编译器,这里的代码可能不安全,需要程序员自己保证安全
  - 一般来说,`unsafe`代码块中的代码应该尽量少,并且尽量简单
- Unsafe Rust中可以执行的4个动作(超能力):
  - 解引用裸指针
  - 调用不安全函数或方法
  - 访问或修改可变静态变量
  - 实现不安全trait
- 注意: Unsafe代码并没有关闭借用检查或停用其他安全检查
  - 如果你在unsafe代码块中使用引用,仍然会进行借用检查
    - `unsafe`这个关键字仅仅是让你可以访问以上4种不会被编译器进行安全检查的特性
    - 因此,即便处于`unsafe`代码块中,你仍然可以获得一定的安全性
  - 通过把不安全的代码留在`unsafe`代码块中,使得任何内存安全相关的错误都必须留在unsafe块中
  - 尽可能隔离unsafe代码,最好将其封装在安全的抽象里,提供安全的API
    - 标准库中的某些功能就使用了unsafe代码,但是在这些unsafe代码的基础上,提供了安全的API,这就可以有效防止unsafe代码泄露到任何调用它的地方

## 2.1 解引用裸指针

在unsafe Rust中,有2种类似于引用的新型指针,它们是裸指针(raw pointer).和引用类似,裸指针也是指向某个值的指针,**但是裸指针不受借用规则约束,可以同时拥有可变和不可变指针,或者是空指针**

- 可变的原始指针:`*mut T`
  - 注意:**这里的`*`是类型名的一部分,不是解引用操作符**
- 不可变的原始指针: `*const T`
  - 不可变的原始指针意味着指针在解引用后不能直接对其进行赋值

- 原始指针与引用的不同:
  - 原始指针可以忽略借用规则:原始指针可以同时具有不可变指针和可变指针;或者多个指向同一位置的可变指针
  - 原始指针无法保证能指向合理的内存;而引用则总是指向有效(合理)的内存
  - 原始指针可以为空;而引用则总是非空的
  - 原始指针不实现任何自动清理
- 放弃保证的安全,以换取更好的性能/与其他语言交互的能力

例: 基于有效的引用创建原始指针

```rust
fn main() {
  let mut num = 5;

  // 注意: 这段不安全代码并没有存在于unsafe代码块中
  // 但是 只能在unsafe代码块中对原始指针进行解引用
  // 这2个原始指针是来自一个有效的引用(&num) 所以我们可以确定这2个指针也是有效的
  // 但原始指针并不总是有效的 所以对原始指针进行解引用的操作必须放在unsafe代码块中
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    // 对原始指针的解引用需要放在unsafe代码块中
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}
```

```
 cargo check
    Checking dereference_raw_pointer v0.1.0 (/dereference_raw_pointer)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
```

注:在Rust中,`cargo check`命令的主要作用是快速检查代码的语法和类型错误,而不生成可执行文件或库.它会进行以下操作:

1. **编译检查**:`cargo check`会运行编译器,但不会生成最终的可执行文件或库.这意味着它会检查代码中的语法错误、类型错误以及其他编译时错误
2. **速度快**:由于`cargo check`不会执行完整的编译过程,它的运行速度比 `cargo build` 更快,非常适合在开发过程中频繁使用,以快速发现和修复错误
3. **节省资源**:`cargo check`仅进行检查,不会生成二进制文件,这样可以节省磁盘空间和编译时间

总结起来,`cargo check`是一个用于在开发过程中快速发现代码问题的有用工具.它可以帮助开发者在编写代码时保持高效,并尽早发现潜在的错误


```
dereference_raw_pointer % cargo run
   Compiling dereference_raw_pointer v0.1.0 (/dereference_raw_pointer)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.07s
     Running `target/debug/dereference_raw_pointer`
r1 is: 5
r2 is: 5
```

例: 无效的原始指针

```rust
fn main() {
  let address = 0x12345usize;
  let r = address as *const i32;

  unsafe {
    println!("r is: {}", *r);
  }
}
```

```
cargo check
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
dereference_raw_pointer_2 % cargo run
```

可以看到,这段代码是能够通过编译的

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/dereference_raw_pointer_2`
zsh: segmentation fault  cargo run
```

- `segmentation fault`: 段错误,是一种程序错误,通常是由于程序访问了不属于它的内存区域,或者试图对只读内存进行写操作,或者内存溢出等原因导致的

2个原因导致的段错误:

1. 段错误(Segmentation Fault): 操作系统保护内存空间,防止程序访问未授权的内存地址.当你访问了无效的内存地址时,操作系统会终止程序,并抛出段错误.段错误不是Rust的错误信息,而是操作系统报告的错误
2. 无效地址:`0x12345`这个地址几乎肯定不是你的程序可以合法访问的内存地址.现代操作系统对内存管理非常严格,随意访问内存通常会导致段错误

为什么要使用原始指针?

- 与C语言进行交互
- 构建一些借用检查器无法理解的安全抽象

## 2.2 调用不安全函数或方法

- unsafe函数或方法: 在定义前加上了`unsafe`关键字的函数或方法
  - 调用这种函数或方法前,需要手动满足一些条件(主要靠看这些函数或方法的文档),因为Rust无法对这些条件进行验证
  - 需要在unsafe块中进行调用

例: 定义并调用unsafe函数

```rust
fn main() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("This is an unsafe function");
}
```

```
cargo run
   Compiling call_unsafe_fn v0.1.0 (/call_unsafe_fn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/call_unsafe_fn`
This is an unsafe function
```

注意:调用unsafe函数的代码必须放在unsafe代码块中,否则会报错

### 2.2.1 创建unsafe代码的安全抽象

- 函数包含unsafe代码,并不意味着需要将整个函数标记为unsafe
- 将unsafe代码包裹在安全函数中,是一个常见的抽象

例: 使用`split_at_mut()`这个unsafe函数创建一个安全的函数

`split_at_mut()`: 是定义在可变切片类型上的,该函数用于在给定的索引位置切割切片,并返回一个元组,该元组中包含了切割后的2个切片

实际上这个函数就调用了一些unsafe代码

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(b, &[4, 5, 6]);
}
```

```
cargo run
   Compiling security_abstraction v0.1.0 (/security_abstraction)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/security_abstraction`
```

假如我们要自己实现一个`split_at_mut()`函数,我们可以这样做:

```rust
fn main() {
  let mut v = vec![1, 2, 3, 4, 5, 6];
  let (left, right) = split_at_mut(&mut v, 3);
  assert_eq!(left, &mut [1, 2, 3]);
  assert_eq!(right, &mut [4, 5, 6]);
}


fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
  let len = slice.len();

  // 判断用于切割的索引是否合法
  assert!(mid <= len);

  // 此处对slice进行了两次可变引用,这是借用检查器不允许的
  (&mut slice[..mid], &mut slice[mid..])  // error: cannot borrow `*slice` as mutable more than once at a time
}
```

```
cargo run
   Compiling split_at_mut_example v0.1.0 (/split_at_mut_example)
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
  --> src/main.rs:15:30
   |
9  | fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
   |                        - let's call the lifetime of this reference `'1`
...
15 |     (&mut slice[..mid], &mut slice[mid..])
   |     -------------------------^^^^^--------
   |     |     |                  |
   |     |     |                  second mutable borrow occurs here
   |     |     first mutable borrow occurs here
   |     returning this value requires that `*slice` is borrowed for `'1`

For more information about this error, try `rustc --explain E0499`.
error: could not compile `split_at_mut_example` (bin "split_at_mut_example") due to 1 previous error
```

从借用规则上而言,确实不能对同一个数据进行2次可变借用.但实际上应该是可以的,因为这2次可变借用,借用的是这个数据的不同部分,并不会发生数据竞争

这时,就需要使用unsafe代码来实现这个函数

- `as_mut_ptr()`: 返回一个指向切片第一个元素的原始指针
- `from_raw_parts_mut()`: 该方法接收的第1个参数是一个原始指针,第2个参数是切片的长度.表示从原始指针处开始,创建一个指定长度的切片
- `add()`: 该方法接收一个usize类型的参数,表示将原始指针向前偏移多少个元素的长度

```rust
use std::slice::from_raw_parts_mut;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            from_raw_parts_mut(ptr, mid),
            from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
```

```
cargo run
   Compiling split_at_mut_example_2 v0.1.0 (/split_at_mut_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/split_at_mut_example_2`
```

在我们自己实现的`split_at_mut()`函数中,确实使用了unsafe代码,但是这个函数本身并没有被标记为unsafe.这就是unsafe代码的安全抽象

### 2.2.2 使用`extern`函数调用外部代码

- `extern`关键字: 用于创建和使用外部函数接口(FFI, Foreign Function Interface)的过程
  - 任何在extern块中声明的函数都是不安全的
  - 因为其他语言并不会强制执行Rust的内存安全规则,而Rust也无法对它们进行检查,所以Rust无法保证这些函数是安全的
  - `extern`关键字后面可以跟`"C"`或`"system"`等字符串,表示调用C语言的库
  - `extern`关键字后面还可以跟`crate`关键字,表示调用其他Rust代码
- 外部函数接口(FFI):它允许一种编程语言定义函数,并让其他编程语言能调用这些函数

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        let abs_value = abs(-3);
        println!("Absolute value of -3 according to C: {}", abs_value);
    }
}
```

- 应用二进制接口(ABI, Application Binary Interface): 定义函数在汇编层面的调用方式
- "C" ABI是最常见的ABI,它遵循C语言的ABI

### 2.2.3 从其他语言调用Rust函数

- 可以使用`extern`创建接口,其他语言通过它们可以调用Rust的函数
- 在`fn`前添加`extern`关键字,并指定对应的ABI
- 还需添加`#[no_mangle]`注解:避免Rust在编译时改变它的名称
  - 所谓mangle,指的是编译过程中的一个阶段,在这个阶段,编译器会将函数名和参数名转换为一种特定的格式,以便在链接时能够正确地找到对应的函数
  - 为了让其他语言能够正常识别Rust的函数,需要禁用这个过程,即添加`#[no_mangle]`注解

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
}
```

这样的函数不需要unsafe标记

## 2.3 访问或修改一个可变静态变量

- Rust支持全局变量,但因为所有权机制可能产生某些问题,比如数据竞争
- 在Rust中,全局变量叫做静态(static)变量

```rust
// 使用static关键字声明全局变量
// 全局变量要求使用大写字母和下划线组成,且必须声明类型
// 静态变量只能存储拥有'static生命周期的引用 ('static生命周期是整个程序运行期间的生命周期)
// 这也就意味着编译器能够推断出全局变量的生命周期 无需手动标注
// 因此无需写成 static HELLO_WORLD: &'static str = "Hello, world!"; 这样的形式
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("{}", HELLO_WORLD);
}
```

### 2.3.1 静态变量

- 静态变量与常量类似
- 命名规范: `SCREAMING_SNAKE_CASE`
- 必须标注类型
- 静态变量只能存储`'static`生命周期的引用,且无需手动标注`'static`生命周期
- 访问不可变的静态变量是安全的

### 2.3.2 常量和不可变静态变量的区别

- 静态变量: 有固定的内存地址,使用它的值总会访问到相同的数据
- 常量: 允许使用它们的时候对数据进行复制
- 静态变量: 可以是可变的,访问和修改静态可变变量是不安全(unsafe)的
- 常量: 不允许对其值进行修改

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
  // 修改静态变量是不安全的操作
  unsafe {
    COUNTER += inc;
  }
}

fn main() {
  add_to_count(3);
  // 访问静态变量同样也是不安全的操作
  unsafe {
    println!("COUNTER: {}", COUNTER);
  }
}
```

```
cargo run
   Compiling static_variable_example_2 v0.1.0 (/static_variable_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
     Running `target/debug/static_variable_example_2`
COUNTER: 3
```

本例中,访问和修改操作都是单线程串行的,所以不会出现数据竞争

但如果是多线程的,那么就会出现数据竞争,因此访问和修改静态变量是不安全的操作

## 2.4 实现不安全(unsafe) trait

- 当某个trait中存在至少1个方法拥有编译器无法校验的不安全因素时,称该trait为不安全trait
- 声明unsafe trait: 在定义前加上`unsafe`关键字
  - 该trait只能在unsafe代码块中实现

```rust
unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {}
```

## 2.5 何时使用unsafe代码

- 编译器无法保证内存安全,保证unsafe代码正确并不简单
- 有充足理由使用unsafe代码时,就可以这样做
- 通过显式标记unsafe代码块,可以在出现问题时轻松定位