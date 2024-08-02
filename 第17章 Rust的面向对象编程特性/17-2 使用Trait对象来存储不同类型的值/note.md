# PART1. 需求

- 创建一个GUI工具
  - 它会遍历GUI元素的列表,依次调用GUI元素的`draw()`方法进行绘制
  - 例如: Button、TextField等元素

- 在面向对象语言中,通常使用继承来实现该需求
  - 定义一个Component父类,该父类中定义了`draw()`方法
  - 定义Button、TextField等子类,并继承Component父类

# PART2. 为共有行为定义一个Trait

Rust中是没有继承的,但是可以使用Trait来实现类似的功能

- Rust中,避免将struct或enum称为对象,因为它们与impl块是分开的
  - 也就是说,struct或enum定义了数据,而impl块定义了方法,数据和方法是分开的
- Rust中,Trait对象有些类似于其他语言中的对象:
  - 它们在某种程度上组合了数据和行为
- 但是,Trait对象也有与传统对象不同的地方:
  - 无法为Trait对象添加数据
    - 可以理解为Trait本身不能存储数据,只能存储行为
- Trait对象被专门用于抽象某些共有行为,它没有其他语言中的对象那么通用

# PART3. `dyn`关键字

- `dyn`关键字用于于指代动态分发的特征对象(trait object)
- 特征对象(trait object): 一种允许在**运行时**进行动态分发的方式

使用`dyn`关键字来修改上一节中小猫小狗的例子:

```rust
trait Animal {
    fn name(&self) -> String;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// Sized特性表示类型的大小在编译时是已知的
// ?Sized表示类型的大小在编译时是未知的
// 这里的T是指Animal的实现类型,所以T是大小未知的,因此要使用?Sized
fn make_animal_talk<T: Animal + ?Sized>(animals: Vec<Box<T>>) {
    for animal in animals {
        animal.talk();
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Dog"),
    };
    let cat = Cat {
        name: String::from("Cat"),
    };

    // 使用dyn关键字进行动态分发
    // 注意: 这里直接写Vec<dyn Animal>会报错,因为dyn Trait是一个动态大小类型,需要使用Box来包装
    // 因为Rust要求所有的类型在编译时都需要知道其大小,而dyn Trait是一个动态大小类型,所以需要使用Box来包装
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    make_animal_talk(animals);
}
```

```
cargo run
   Compiling dyn_example v0.1.0 (/dyn_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/dyn_example`
Dog cannot talk
Cat says meow
```

# PART4. `dyn`关键字和泛型约束的区别

1. 动态分发 vs 静态分发

   - `dyn`关键字:使用`dyn`时,特征对象会通过**动态分发**来调用其方法.这意味着在运行时通过虚表(vtable)查找方法,可能会带来一定的性能开销
     - 动态派发(dynamic dispatch):无法在编译过程中确定你调用的具体方法,因为动态派发是在运行时通过虚表查找方法
     - 编译器会在编译时生成虚表,虚表中存储了特征对象的方法地址,通过虚表可以在运行时查找到具体方法
   - 泛型约束: 使用泛型时,编译器会在编译时进行**静态分发**(单态化),直接调用具体类型的方法.这样可以避免运行时的开销,通常性能更好
     - 所谓单态化,就是指编译器在编译时会使用具型来替换泛型,为每一个具型生成对应的函数和方法的非泛型实现
     - 通过单态化生成的代码,会执行静态派发(static dispatch),在编译过程中确定调用的具体方法

2. 灵活性 vs 性能

   - 灵活性: 特征对象提供了更大的灵活性,可以在运行时处理多种不同类型,而不需要在编译时知道每种具体类型.通常在需要存储不同类型的集合时非常有用
   - 性能: 泛型在编译时确定具体类型,有助于提高性能,因为可以直接调用具体类型的方法而不需要通过虚表.对于性能敏感的代码,泛型通常是更好的选择

3. 类型擦除 vs 类型安全

    - 类型擦除:使用`dyn`时,具体类型信息在编译时被擦除,只有特征的方法可以使用.这意味着我们不能在特征对象上调用具体类型的方法,也不能使用返回`self`类型的方法
      - 由于已经擦除了具型,因此自然不知道具型有什么方法
      - 同理,返回`self`类型的方法也无法使用,因为不知道`self`指代的具型是什么
    - 类型安全:泛型在编译时确定具体类型,可以直接调用具体类型的方法,并且可以使用返回`self`类型的方法.这样可以更好地利用类型系统,提高代码的安全性

# PART5. 使用特征对象实现需求

```
/trait_object_example % tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs

1 directory, 4 files
```

`lib.rs`:

```rust
/// 本Trait用于定义GUI元素的共有行为
pub trait Draw {
    fn draw(&self);
}

/// 本结构体用于表示屏幕 存储所有存在于屏幕中的GUI元素
/// 相比于使用泛型约束 使用特征对象的优点在于可以存储不同类型的GUI元素
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// 本方法用于在屏幕上绘制所有GUI元素
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

`main.rs`:

```rust
use trait_object_example::{Draw, Screen};

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: width={}, height={}, label={}", self.width, self.height, self.label);
    }
}
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: width={}, height={}, options={:?}", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

```
cargo run
   Compiling trait_object_example v0.1.0 (/trait_object_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.60s
     Running `target/debug/trait_object_example`
SelectBox: width=75, height=10, options=["Yes", "Maybe", "No"]
Button: width=50, height=10, label=OK
```

但如果使用泛型约束,则无法存储不同类型的GUI元素:

```
generic_constraint_example % tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs

1 directory, 4 files
```

`lib.rs`:

```rust
/// 本Trait用于定义GUI元素的共有行为
pub trait Draw {
    fn draw(&self);
}

/// 本结构体用于表示屏幕 存储所有存在于屏幕中的GUI元素
/// 使用泛型约束则要求传入的T的具型是相同的 你无法将Button和TextField同时存储在Screen中
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw
{
    /// 本方法用于在屏幕上绘制所有GUI元素
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

`main.rs`:

```rust
use generic_constraint_example::{Draw, Screen};

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: width={}, height={}, label={}", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: width={}, height={}, options={:?}", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {   // error: type mismatch in generic constraints
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

```
cargo run
   Compiling generic_constraint_example v0.1.0 (/generic_constraint_example)
error[E0308]: mismatched types
   --> src/main.rs:39:22
    |
39  |               Box::new(Button {   // error: type mismatch in generic constraints
    |  _____________--------_^
    | |             |
    | |             arguments to this function are incorrect
40  | |                 width: 50,
41  | |                 height: 10,
42  | |                 label: String::from("OK"),
43  | |             }),
    | |_____________^ expected `SelectBox`, found `Button`
    |
...
```

# PART6. 特征对象必须保证对象安全

- 只能把满足对象安全(object-safe)的Trait转化为特征对象
- Rust采用一系列规则来判定某个对象是否安全,只需记住2条:
  - Trait中的方法的返回类型不能包含`Self`
  - Trait中的方法不能有泛型类型参数

例如:标准库中的`Clone` Trait就不符合对象安全的规则:

```rust
pub trait Clone: Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn clone(&self) -> Self;
    
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
```