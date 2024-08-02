# PART1. 元组结构体

元组结构体(tuple struct): 本质上就是在元组类型的基础上自定义了一个新的类型.元组结构体通过将多个值组合在一起形成一个类型,但与普通元组不同的是,它们有一个明确的名称,因此可以为其实现特定的方法和特征

元组结构体的定义语法与元组类似,但具有类型名,这使得它们更具可读性和可维护性.此外,元组结构体可以用作其他类型的构建块,并且可以实现方法和特征.

```rust
struct Color(i32, i32, i32);

impl Color {
    fn describe(&self) {
        // 元组结构体的字段通过索引位置访问
        println!("RGB({}, {}, {})", self.0, self.1, self.2);
    }
}

fn main() {
    let color = Color(255, 165, 0);
    color.describe();
}
```

```
cargo run 
   Compiling tuple_struct v0.1.0 (/tuple_struct)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.95s
     Running `target/debug/tuple_struct`
RGB(255, 165, 0)
```

注意:**自定义的元组结构体类型,它的内存是分配在栈上的**

# PART2. Deref Trait

Deref: 即解引用(de-reference)的意思

- 实现Deref Trait,可以使我们自定义解引用运算符`*`的行为
- 通过实现Deref Trait,使得智能指针可以像常规引用一样处理
  - 也就是说,处理引用的代码,可以不加修改地用于处理智能指针

# PART3. 解引用运算符

- 常规引用是一种指针

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);

    // 此处*的含义为解引用,即取出y指向的值
    assert_eq!(5, *y);
}
```

```
cargo run
   Compiling normal_reference v0.1.0 (/normal_reference)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/normal_reference`
```

# PART4. 把`Box<T>`当作引用

- 使用`Box<T>`替代上个例子中的引用

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

```
cargo run              
   Compiling box_reference v0.1.0 (/box_reference)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/box_reference`
```

可以看到,两段代码中,除了把`y`换成了`Box::new(x)`外,其他代码都是一样的.这说明可以像使用引用那样使用`Box<T>`

# PART5. 定义自己的智能指针

- `Box<T>`被定义成拥有1个元素的元组结构体,这个元素是`T`类型的值

```rust
pub struct Box<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);
```

先来尝试实现:

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

fn main() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y); // error: type `MyBox<{integer}>` cannot be dereferenced
}
```

```
cargo run
   Compiling custom_box v0.1.0 (/custom_box)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y); // error: type `MyBox<{integer}>` cannot be dereferenced
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `custom_box` (bin "custom_box") due to 1 previous error
```

error[E0614]: type `MyBox<{integer}>` cannot be dereferenced: `MyBox`类型不能被解引用.因为我们没有为该类型实现解引用的功能 

这是因为我们没有实现`Deref` Trait,所以不能使用`*`运算符

# PART6. 实现`Deref` Trait

- `Deref` Trait是Rust提供的一个特性,用于重载`*`运算符的行为

```rust
#[lang = "deref"]
#[doc(alias = "*")]
#[doc(alias = "&*")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "Deref"]
pub trait Deref {
    /// The resulting type after dereferencing.
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "deref_target"]
    #[lang = "deref_target"]
    type Target: ?Sized;

    /// Dereferences the value.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "deref_method"]
    fn deref(&self) -> &Self::Target;
}
```

- `Deref` Trait有一个关联类型`Target`,用于指定解引用后的类型(这个后边再讲)
- `Deref` Trait要求我们实现`deref()`方法:
  - 该方法借用`self`
  - 该方法返回一个指向内部数据的引用
  - 由于返回的是一个引用,所以就可以使用`*`运算符来访问引用指向的值了
- 如果没有`Deref` Trait,那么编译器就只能对`&x`这种常规引用使用`*`运算符,而不能对自定义类型使用

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 这里的*y,实际上就相当于是*(y.deref()) 实际上Rust也会把*y隐式地转换成*(y.deref())
}
```

```
 cargo run
   Compiling custom_box v0.1.0 (/custom_box)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/custom_box`
```
