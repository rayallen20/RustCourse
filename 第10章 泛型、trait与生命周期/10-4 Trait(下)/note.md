# PART1. Trait作为参数

## 1.1 impl Trait语法

这种方式适用于简单情况

例:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // 默认实现中调用Trait的另一个方法
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 重载默认实现
    fn summarize(&self) -> String {
        // 重载后的实现没有调用Trait中的另一个方法
        format!("{}: {}", self.username, self.content)
    }
}

// 参数item为实现了Summary Trait的某个类型
// 可以认为是多态(Polymorphism)
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

## 1.2 Trait Bound语法

这种方式可用于复杂情况

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs

1 directory, 4 files
```

lib.rs:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // 默认实现中调用Trait的另一个方法
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 重载默认实现
    fn summarize(&self) -> String {
        // 重载后的实现没有调用Trait中的另一个方法
        format!("{}: {}", self.username, self.content)
    }
}

// Trait Bound写法
// 在比较复杂的情况下, Trait Bound写法更加简洁
// 注意: 在调用notify时, item1和item2的类型必须相同
// 这里的类型相同,不是指item1和item2均为Summary Trait的实现,而是指它们是同样的具型
pub fn notify<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

// impl Trait参数的写法 比Trait Bound写法的方法签名要长
pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}
```

main.rs:

```rust
use trait_bound::Tweet;
use trait_bound::notify;

fn main() {
    let tweet1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // 调用notify()函数时, item1和item2的类型必须相同
    // 我猜测这和编译时的单态化有关
    notify(tweet1, tweet2);
}
```

```
cargo run
   Compiling trait_bound v0.1.0 (/trait_bound)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/trait_bound`
Breaking news! horse_ebooks: of course, as you probably already know, people, horse_ebooks: of course, as you probably already know, people
```

**`impl Trait`语法是Trait bound的语法糖**

## 1.3 使用`+`指定多个Trait Bound

当需要指定1个泛型参数实现多个Trait时, 可以使用`+`指定多个Trait Bound

例:要求泛型`T`实现`Summary`和`Display`两个Trait

```rust
use std::fmt::Display;

// Trait bound语法的多个Trait bound(Trait约束)
pub fn notify<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait语法的多个Trait bound(Trait约束)
pub fn notify2(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}
```

## 1.4 使用where从句简化Trait Bound

以上两种写法,多少还是有一些缺点的.每个泛型都需要指定自己的Trait Bound,当泛型参数较多时(如3个以上),函数名和参数列表中间的Trait Bound部分会很长,不易阅读.

为了解决这个问题,Rust提出的解决方案是:在函数签名的后边,使用`where`从句指定Trait Bound

```rust
use std::fmt::{Debug, Display};

// 可以看到 当有2个泛型参数,且每个泛型参数有2个trait bound时,函数名和形参列表之间的Trait Bound就已经很长了
pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

// 使用where关键字可以使代码更加简洁
pub fn notify_where<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}
```

# PART2. Trait作为返回类型

## 2.1 impl Trait语法

```rust
// 注意: 这里虽然函数签名上写的返回类型为Summary Trait的实现即可
// 但是要求返回值的类型必须是确定的具型,不能是动态的
// 这一点和使用Trait作为参数是相同的.我猜测也是因为单态化的原因
pub fn notify(s: &str) -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(s),
        reply: false,
        retweet: false,
    }
}

// 这里的返回值类型是动态的,所以编译器会报错
// error[E0308]: `if` and `else` have incompatible types
// pub fn notify1(s: &str) -> impl Summary {
//     if s.len() > 10 {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(s),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
```

**注意:`impl Trait`只能返回确定的同一种具型,不能返回泛型的不同实现**

# PART3. 使用Trait Bound修复`find_largest()`函数

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));
    
    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T> (numbers: &[T]) -> T {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest { // // error[E0369]: binary operation `>` cannot be applied to type `T`
            largest = number;
        }
    }

    largest
}
```

在我们之前的实现中,`find_largest()`函数无法编译,因为编译器无法确定`T`类型是否支持`>`操作符.

```
 cargo run
   Compiling repair_largest v0.1.0 (/repair_largest)
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/main.rs:13:19
   |
13 |         if number > largest {
   |            ------ ^ ------- T
   |            |
   |            T
   |
help: consider restricting type parameter `T`
   |
9  | fn find_largest<T: std::cmp::PartialOrd> (numbers: &[T]) -> T {
   |                  ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `repair_largest` (bin "repair_largest") due to 1 previous error
```

报错信息已经提示我们,需要为`T`指定`PartialOrd` Trait Bound

## 3.1 为`T`类型添加`std::cmp::PartialOrd`约束

因为`>`操作符实际上是调用的`PartialOrd` Trait中的`gt()`方法,所以我们需要为`T`指定`PartialOrd` Trait Bound

```rust
// PartialOrd Trait 中定义的gt()方法
fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
```

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T: PartialOrd> (numbers: &[T]) -> T {
    let mut largest = numbers[0]; // error[E0508]: cannot move out of type `[T]`, a non-copy slice

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

```
cargo build
   Compiling repair_largest v0.1.0 (/repair_largest)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
  --> src/main.rs:10:23
   |
10 |     let mut largest = numbers[0]; //
   |                       ^^^^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `numbers[_]` has type `T`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
10 |     let mut largest = &numbers[0]; //
   |                       +

error[E0507]: cannot move out of a shared reference
  --> src/main.rs:12:20
   |
12 |     for &number in numbers {
   |          ------    ^^^^^^^
   |          |
   |          data moved here
   |          move occurs because `number` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
12 -     for &number in numbers {
12 +     for number in numbers {
   |

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `repair_largest` (bin "repair_largest") due to 2 previous errors
```

## 3.2 为`T`类型添加`Copy`约束

报错信息中显示:

```
10 |     let mut largest = numbers[0]; //
   |                       ^^^^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `numbers[_]` has type `T`, which does not implement the `Copy` trait
```

无法从`numbers`中移动`T`类型的值,因为`T`类型没有实现`Copy` Trait

这里多提一嘴,`let mut largest = numbers[0];`这一行代码并没有导致`numbers[0]`的所有权被移动.

真正导致它的所有权被移动,是因为函数的签名:`fn find_largest<T: PartialOrd + Copy> (numbers: &[T]) -> T`

`find_largest()`返回一个`T`类型的值,而这个函数最终的返回值`largest`是从`numbers`中取出的`numbers[0]`的值.当函数执行完毕,最终返回`largest`时,`largest`的所有权会被移动,而`numbers[0]`的所有权也会被移动.

因此报错信息也提到,可以将返回值的类型改为`&T`,这样就不会移动`numbers[0]`的所有权了

这里我们采用另外一种方案:再给`T`指定`Copy` Trait Bound

这里之所以我们能够使用`Copy` Trait,是因为至少目前,我们看到的需求是针对`Vec<i32>`和`Vec<char>`这两种类型,实现`find_largest()`函数

而i32和char这两种类型都是标量,它们的数据都是分配在Stack上的,分配在Stack上的数据类型都是实现了`Copy` Trait的

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T: PartialOrd + Copy> (numbers: &[T]) -> T {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

```
cargo run
   Compiling repair_largest v0.1.0 (/repair_largest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/repair_largest`
The largest number in numbers1 is 100
The largest char in chars1 is y
```

虽然现在实现了需求,但如果我们要是还有其他类型的需求,比如`Vec<String>`,那么这个函数就无法满足了

## 3.3 使用`Clone` Trait替代`Copy` Trait

回顾: `Copy` Trait是复制Stack上的数据,而`Clone` Trait是复制Heap上和Stack上的数据

String类型是实现了`Clone` Trait的,所以我们可以使用`Clone` Trait替代`Copy` Trait

注意:Rust中所有的基础类型都实现了Clone Trait,所以可以使用Clone Trait替代Copy Trait

```rust
fn find_largest<T: PartialOrd + Clone> (numbers: &[T]) -> T {
    let mut largest = numbers[0]; // // cannot move out of here

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

```
cargo build
   Compiling repair_largest v0.1.0 (/repair_largest)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
  --> src/main.rs:13:23
   |
13 |     let mut largest = numbers[0];
   |                       ^^^^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `numbers[_]` has type `T`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
13 |     let mut largest = &numbers[0];
   |                       +

error[E0507]: cannot move out of a shared reference
  --> src/main.rs:15:20
   |
15 |     for &number in numbers {
   |          ------    ^^^^^^^
   |          |
   |          data moved here
   |          move occurs because `number` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
15 -     for &number in numbers {
15 +     for number in numbers {
   |

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `repair_largest` (bin "repair_largest") due to 2 previous errors
```

这里编译不通过的原因其实和之前的原因是一样的,还是因为返回值的类型为`T`,这导致了`largest`的所有权被移动,而`numbers[0]`的所有权也被移动

解决办法也是和之前相同:返回`&T`类型;或者返回`numbers[0]`的克隆值

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));

    let strings1: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    println!("The largest string in strings1 is {}", find_largest(&strings1));
}

fn find_largest<T: PartialOrd + Clone> (numbers: &[T]) -> T {
    let mut largest = numbers[0].clone();

    // 这里不再需要使用&number来匹配numbers中的元素,因为切片是不可变引用
    // 而最终的返回值是T类型,所以需要使用clone()方法来复制元素
    for number in numbers {
        // 这里需要解引用number,因为number是一个引用
        if *number > largest {
            largest = number.clone();
        }
    }

    largest
}
```

```
cargo run
   Compiling repair_largest v0.1.0 (/repair_largest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/repair_largest`
The largest number in numbers1 is 100
The largest char in chars1 is y
The largest string in strings1 is world
```

Tips:这里我的实现和书中的是不一样的,书中原来的实现是:

```rust
fn find_largest<T: PartialOrd + Clone> (numbers: &[T]) -> T {
    let mut largest = numbers[0].clone();
    
    for &number in numbers.iter() { // data moved here
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

解决办法就是不让`&number`的所有权被移动,使用`number`即可(因为`&number`的类型为`T`,而`number`的类型为`&T`)

```rust
fn find_largest<T: PartialOrd + Clone> (numbers: &[T]) -> T {
    let mut largest = numbers[0].clone();

    for number in numbers.iter() {
        // number的类型为&T 所以这里需要解引用
        if *number > largest {
            // 赋值时同样也是为了避免所有权的移动 将number的clone赋值给largest
            largest = number.clone();
        }
    }

    largest
}
```

# PART4. 使用Trait Bound有条件的实现方法

在使用泛型类型参数的impl块上使用Trait Bound,可以有条件的为实现了特定Trait Bound的类型实现方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 无论泛型T为何种具型 均实现了关联函数new()
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 为实现了Display和PartialOrd的泛型T实现cmp_display()方法
// 所谓有条件的实现方法,是指只有当泛型T满足条件:T既实现了Display Trait,又实现了PartialOrd Trait时
// 才为Pair<T>实现cmp_display()方法
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
```

也可以为实现了其他Trait的任意类型有条件的实现某个Trait

覆盖实现(blanket implementation):给所有实现了特定Trait的类型,实现某个Trait

标准库中,string.rs中的`impl<T: fmt::Display + ?Sized> ToString for T`就是一个例子

```rust
// 为所有实现了Display Trait的类型T 实现ToString Trait
impl<T: fmt::Display + ?Sized> ToString for T {/*...*/}
```

例如:i32类型实现了Display Trait,所以i32类型也实现了ToString Trait

```rust
fn main() {
    // i32类型实现了Display trait
    // 因此 i32类型也实现了ToString trait
    let s = 3.to_string();
}
```
