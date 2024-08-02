# PART1. 在Trait定义中使用关联类型来指定占位类型

- 关联类型(associated type): 是Trait中的类型占位符,它可以用于Trait的方法签名中
  - 可以定义出包含某些类型的Trait,而在实现这个Trait之前无需确定这些类型
  - 这一点和泛型不同,泛型Trait需要在实现时,指定具体的类型

```rust
trait Iterator {
    // 标准库中的Iterator Trait大致定义如下
    // 此处的Item就是关联类型 用以指代迭代时 返回的元素类型
    // 这个Item可以认为就是一种类型占位符
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("Hello, world!");
}
```

## 1.1 关联类型与泛型的区别

- 泛型
  - 每次实现泛型Trait时,都需要标注泛型的具型
  - 可以为同一个类型多次实现泛型Trait,只要这些实现的泛型具型不同即可

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    count: u32,
}

// 需要标注泛型的具型
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(self.count)
    }
}

// 可以为同一个类型多次实现泛型Trait
impl Iterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        let res = self.count.to_string();
        match res {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
```

- 关联类型
  - 实现Trait时无需标注类型
    - 但是要在实现Trait的代码块中指定关联类型
  - 无法为单个类型多次实现关联类型Trait

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
  count: u32,
}

// 无需指定关联类型
impl Iterator for Counter {
  // 需要在实现中指定关联类型
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.count)
  }
}

// 不能为同一个类型实现多次相同的trait
impl Iterator for Counter {     // error: conflicting implementations of trait `Iterator` for type `Counter`
  type Item = String;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.count.to_string())
  }
}

fn main() {
  println!("Hello, world!");
}
```

```
cargo run
   Compiling associated_type_trait v0.1.0 (/associated_type_trait)
error[E0119]: conflicting implementations of trait `Iterator` for type `Counter`
  --> src/main.rs:20:1
   |
11 | impl Iterator for Counter {
   | ------------------------- first implementation here
...
20 | impl Iterator for Counter {     // error: conflicting implementations of trait `Iterator` for type `Counter`
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Counter`

error[E0282]: type annotations needed
  --> src/main.rs:14:5
   |
14 |     fn next(&mut self) -> Option<Self::Item> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
note: the requirement `_ <: _` appears on the `impl`'s method `next` but not on the corresponding trait's method
  --> src/main.rs:3:8
   |
1  | trait Iterator {
   |       -------- in this trait
2  |     type Item;
3  |     fn next(&mut self) -> Option<Self::Item>;
   |        ^^^^ this trait's method doesn't have the requirement `_ <: _`

error[E0282]: type annotations needed
  --> src/main.rs:22:5
   |
22 |     fn next(&mut self) -> Option<Self::Item> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
note: the requirement `_ <: _` appears on the `impl`'s method `next` but not on the corresponding trait's method
  --> src/main.rs:3:8
   |
1  | trait Iterator {
   |       -------- in this trait
2  |     type Item;
3  |     fn next(&mut self) -> Option<Self::Item>;
   |        ^^^^ this trait's method doesn't have the requirement `_ <: _`

Some errors have detailed explanations: E0119, E0282.
For more information about an error, try `rustc --explain E0119`.
error: could not compile `associated_type_trait` (bin "associated_type_trait") due to 3 previous errors
```

# PART2. 默认泛型参数和运算符重载

- 可以在使用关联类型的Trait中定义默认的泛型参数
  - 这样在实现Trait时,就可以不指定泛型参数的具型
- 语法: `<PlaceholderType=ConcreteType>`
- 这种技术常用于运算符重载(Operator Overloading)
- Rust不允许创建自己的运算符或重载任意运算符
- 但是,可以通过实现`std::ops`中列出的Trait来重载一部分对应的运算符

## 2.1 重载`Add`运算符

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 4, y: 6 });
}
```

```
cargo run
   Compiling default_generic_parameter v0.1.0 (/default_generic_parameter)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/default_generic_parameter`
```

## 2.2 默认参数类型

我们来看一下`std::ops::Add`这个Trait的定义:

```rust
pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 + 1, 13);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

注意这里的`Add<Rhs = Self>`,这里的`Rhs = Self`就是默认参数类型.表示如果没有指定`Rhs`的具体类型,那么就默认为实现该Trait的具型

## 2.3 例:让毫米和米相加

```rust
use std::ops::Add;

/// 本类型表示毫米
struct Millimeters(i32);

// 指定rhs的类型默认为Meters
impl Add<Meters> for Millimeters {
    // 指定关联类型Output为Millimeters
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

/// 本类型表示米
struct Meters(i32);

fn main() {
    let millimeters = Millimeters(1000);
    let meters = Meters(1);
    let result = millimeters + meters;
    assert_eq!(result.0, 2000);
}
```

注意:指定默认的泛型参数类型,与关联类型无关,这两个概念是不同的.关联类型可以理解为一种占位符,而默认泛型参数类型则是为了方便使用者在实现Trait时,不用指定泛型参数的具型

## 2.4 默认泛型参数的主要应用场景

- 扩展一个类型的同时,不破坏现有代码
- 允许当大部分用户都不需要指定泛型参数的特定场景下,进行自定义泛型参数的指定(刚才毫米和米相加的例子就属于这种情况)

# PART3. 完全限定语法(Fully Qualified Syntax)如何调用同名方法

## 3.1 同名方法的调用

```rust
/// 飞行员Trait
trait Pilot {
    fn fly(&self);
}

/// 巫师Trait
trait Wizard {
    fn fly(&self);
}

struct Human;

// Human实现Pilot
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// Human实现Wizard
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human实现自己的fly方法
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;

    // 调用Human自己的fly方法
    person.fly();

    // 调用Human实现了的Pilot的fly方法
    // 这里传&person是因为Pilot的fly方法的接收者是&self
    Pilot::fly(&person);

    // 调用Human实现了的Wizard的fly方法
    Wizard::fly(&person);
}
```

```
 cargo run
   Compiling fully_qualified_example_1 v0.1.0 (/fully_qualified_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/fully_qualified_example_1`
*waving arms furiously*
This is your captain speaking.
Up!
```

## 3.2 完全限定语法

- 完全限定语法(Fully Qualified Syntax): 用于调用同名方法
  - 语法: `<Type as Trait>::function(receiver_if_method, next_arg, ...);`
  - 可以在任何调用函数或方法的地方使用
  - 允许忽略那些从其他上下文能推导出来的部分
  - 当Rust无法区分你期望调用哪个具体实现时,才需要使用这种语法

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // 调用Dog自身的baby_name关联函数
    let dog_baby_name = Dog::baby_name();
    println!("A baby dog is called a {}", dog_baby_name);

    // 调用Dog实现的Animal trait的baby_name方法
    // 上一个例子中,Trait的方法可以通过传入的具型来判断调用该泛型的哪个实现的方法
    // 但是这里由于方法没有接收者,和关联函数一样,编译器无法判断调用哪个具型上的方法
    // 因此需要指明具型
    let animal_baby_name = <Dog as Animal>::baby_name();
    println!("A baby dog is called a {}", animal_baby_name);
}
```

```
cargo run
   Compiling fully_qualified_example_2 v0.1.0 (/fully_qualified_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/fully_qualified_example_2`
A baby dog is called a Spot
A baby dog is called a puppy
```

# PART4. 使用super trait来要求trait附带其他trait的功能

其实就是要求一个Trait继承另一个Trait

- 需要在一个trait中使用其他trait的功能时:
  - 需要被依赖的trait也被该trait的实现者实现
  - 被间接依赖的trait就是当前trait的super trait

```rust
use std::fmt;
use std::fmt::Display;

/// 该Trait用于打印一个图形的轮廓 但该Trait要求其实现者必须实现Display Trait
/// 也就是说该Trait的实现必须实现Display Trait
/// :后即为该Trait依赖的Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 具型要想成为OutlinePrint的实现者 必须成为Display Trait的实现者
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
```

```
cargo run
   Compiling super_trait_example v0.1.0 (/super_trait_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/super_trait_example`
**********
*        *
* (1, 3) *
*        *
**********
```

# PART5. 使用new type模式来实现外部trait的实现

- 孤儿规则: 只有当trait或类型定义在本地时,才能为该类型实现trait
  - 也就是说,如果你定义了一个trait,你可以为任何类型实现这个trait
  - 但是如果你使用的是外部的trait,只有当你定义了这个类型时,才能为这个类型实现这个外部的trait
- 但是可以通过new type模式来实现外部trait的实现
  - new type模式: 定义一个新的类型来包装原有的类型,从而满足孤儿规则
  - 具体的做法就是使用一个tuple struct(元组结构体),将原有的类型包装起来,然后为这个新的类型实现外部的trait

```rust
use std::fmt::Display;

/// 想要为Vector这个外部的类型实现Display 这个外部的Trait
/// 使用new type模式 将Vec<String>包装成一个新的类型Wrapper
/// 然后为Wrapper实现Display
struct Wrapper(Vec<String>);

impl Display for Wrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn main() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
```

```
cargo run          
   Compiling new_type_example v0.1.0 (/new_type_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/new_type_example`
w = [hello, world]
```