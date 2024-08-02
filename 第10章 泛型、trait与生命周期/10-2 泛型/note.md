# PART1. 泛型的能力

泛型: 提高代码的复用能力,可用于处理重复代码的问题

泛型是具体类型或其他属性的抽象代替:

- 你编写的代码并不是最终的代码,而是一种**模板**,这个模板里有一些**占位符**(也就是之前看到的`<T>`)
- 编译器在编译时,将这些占位符替换为具体的类型,这个过程叫做**泛型擦除**(generic erasure)或**单态化**(monomorphization)
  - 例如: `fn largest<T> (numbers: &[T]) -> T {...}`
    - 这段代码中,`T`即为占位符
    - 当编译器编译这段代码时,会将`T`替换为具体的类型
    - 这个`T`称为**类型参数**(type parameter),可以使用任何标识符作为类型参数
      - 在选择泛型的类型参数名时,通常都很短,短到只有一个字母
      - Rust中使用CamelCase作为类型参数的命名规范
      - T正好也是type的首字母,所以T是最常用的类型参数名

# PART2. 函数定义中的泛型

当使用泛型来定义一个函数时,需要将类型参数放在函数的签名中,这样编译器才知道这个函数是泛型的

这个类型参数通常是用于指定参数类型和返回类型的

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    // 如果将集合替换为字符切片,即寻找字符切片中的最大字符
    // 这个场景下就可以使用泛型来实现
    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T> (numbers: &[T]) -> T {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

这段代码编译会报错:

```bash
cargo build
   Compiling fn_genericity v0.1.0 (/fn_genericity)
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/main.rs:18:19
   |
18 |         if number > largest {
   |            ------ ^ ------- T
   |            |
   |            T
   |
help: consider restricting type parameter `T`
   |
14 | fn find_largest<T: std::cmp::PartialOrd> (numbers: &[T]) -> T {
   |                  ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `fn_genericity` (bin "fn_genericity") due to 1 previous error
```
error[E0369]: binary operation `>` cannot be applied to type `T`: 操作符`>`不能应用于类型`T`

提示信息中表明,要限制类型参数`T`为`std::cmp::PartialOrd` trait的实现.

换言之,不是所有的类型都能进行比较大小的操作,只有实现了`std::cmp::PartialOrd` trait的类型才能进行比较大小的操作

因此需要在函数签名中指定`T`的类型:

```rust
use std::cmp::PartialOrd;

fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    // 如果将集合替换为字符切片,即寻找字符切片中的最大字符
    // 这个场景下就可以使用泛型来实现
    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T: PartialOrd> (numbers: &[T]) -> T {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

这样改完了之后,编译还是无法通过:

```bash
cargo build
   Compiling fn_genericity v0.1.0 (/fn_genericity)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
  --> src/main.rs:14:23
   |
14 |     let mut largest = numbers[0];
   |                       ^^^^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `numbers[_]` has type `T`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
14 |     let mut largest = &numbers[0];
   |                       +

error[E0507]: cannot move out of a shared reference
  --> src/main.rs:16:20
   |
16 |     for &number in numbers {
   |          ------    ^^^^^^^
   |          |
   |          data moved here
   |          move occurs because `number` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
16 -     for &number in numbers {
16 +     for number in numbers {
   |

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `fn_genericity` (bin "fn_genericity") due to 2 previous errors
```

这个错误的原因暂时先不讲,重点是演示泛型在函数定义中的使用

# PART3. Struct定义中的泛型

Struct的泛型类型参数主要用于定义Struct的字段类型

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}
```

当然,也可以定义多个泛型类型参数:

- 但是,如果你的Struct中有太多的类型参数,会导致你的代码难以阅读.这时通常要考虑把你的代码重组为多个更小的单元

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 1.4 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);

    let p2 = Point { x: 1.2, y: 4 };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}
```

注意,这个例子中,`T`和`U`的类型也是可以相同的:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 1 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);

    let p2 = Point { x: 1.2, y: 4.4 };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}
```

# PART4. Enum定义中的泛型

Enum中使用泛型类型参数,主要是用在它的变体中

- 例如: `Option<T>`,`Result<T, E>`

`Option<T>`的定义:

```rust
pub enum Option<T> {
    // 并非所有变体都要使用泛型类型参数
    None,
    Some(T),
}
```

`Result<T, E>`的定义:

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

# PART5. 方法定义中使用泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

// impl<T> 表示代码块中的方法是针对泛型T的,而非某个具型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 也可以针对具体的类型来实现方法
// 针对具型实现方法时, impl关键字后边就不需要再写<T>了
// 注意: 这些方法只属于Point<i32>这个具型,其他类型的Point没有这些方法
impl Point<i32> {
    fn x1(&self) -> i32 {
        self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

注意:

- 把`T`放在`impl`关键字后边,表示这些方法是针对泛型`T`的,而非某个具型
- 只针对具型实现的方法,则只属于这个具型;其他类型没有实现这些方法
- Struct中的泛型类型参数可以和方法的泛型类型参数不同

例:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // mixup()方法的泛型参数V, 和Point的泛型参数T没有关系(也就是说这二者的类型可能相同,也可能不同)
    // 同理, W和U也没有关系

    // 参数列表中的self是Point<T, U>类型的, 而other是Point<V, W>类型的 这二者的泛型参数是不同的

    // 返回值列表中的Point<T, W>中的W是和other的泛型参数W相同的, 而T和U是和self的泛型参数T和U相同的
    // 也就是说,泛型方法返回的泛型Point,可以和Struct定义的泛型Point是不同的
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}
```

```bash
cargo run
   Compiling different_method_and_struct_genericity v0.1.0 (/different_method_and_struct_genericity)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/different_method_and_struct_genericity`
p3.x = 5, p3.y = c
```

# PART6. 泛型代码的性能

使用泛型的代码和使用具型的代码运行速度是一样的.因为Rust在编译时会将泛型代码单态化(performance),也就是将泛型代码转换为具型代码

```rust
// fn main() {
//     let integer: Option<i32> = Some(5);
//     let float: Option<f64> = Some(2.0);
// }

// 编译器在编译这段代码时,会读取Option<T>中使用过的值,在本例中,进而编译器确定了2种类型:

// 泛型定义的展开
enum Option_i32 {
    Some(i32),
    None,
}

// 泛型定义的展开
enum Option_f64 {
    Some(f64),
    None,
}

// 单态化后的main()函数,编译器会将Option<T>中的T替换为具体的类型
fn main() {
    let integer: Option_i32 = Option_i32::Some(5);
    let float: Option_f64 = Option_f64::Some(2.0);
}
```