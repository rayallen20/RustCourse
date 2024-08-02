# PART1. 定义方法

方法与函数的不同之处:

- 方法是在struct(enum、trait)的上下文中定义
- 方法的第一个参数是self,代表调用该方法的实例

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl关键字用于定义方法
impl Rectangle {
    // 方法的第1个参数总是self,表示调用该方法的实例
    // &self可以被推断为&Rectangle 且它是一个借用(不可变引用)
    // 这里可以是 self &self &mut self 取决于具体需求
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
    println!("rect is {:#?}", rect);
}
```

# PART2. 方法调用的运算符

C/C++: object->method() 或 (*object).method()

Rust中`->`运算符是用于标识返回值类型的

Rust会自动引用或解引用

在调用方法时,Rust根据情况自动添加&、&mut或*

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
    // 通过引用调用方法 和 通过实例调用方法 二者等效
    println!("{}", (&rect).area());
    println!("rect is {:#?}", rect);

    let rect2: &Rectangle = &Rectangle {
        width: 30,
        height: 50,
    };

    // 通过引用调用方法 和 通过实例调用方法 二者等效
    println!("{}", rect2.area());
    println!("rect2 is {:#?}", rect2);
}
```

```bash
cargo run
   Compiling call_method v0.1.0 (/call_method)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/call_method`
1500
1500
rect is Rectangle {
    width: 30,
    height: 50,
}
1500
rect2 is Rectangle {
    width: 30,
    height: 50,
}

```

二者完全等效

# PART3. 方法参数

方法可以接受多个参数,但第一个参数必须是self

例:判断一个长方形是否可以容纳另一个长方形

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}
```

## PART4. 关联函数

在impl块中,不把self作为第1个参数的函数,称为关联函数.关联函数不是方法,因为它们不作用于一个实例.

但是关联函数通常与struct还是有一定关联的,因此它们通常用于实现构造函数

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 关联函数使用::调用
    let square = Rectangle::square(3);
    println!("{:#?}", square.width);
    println!("{:#?}", square.height);
}
```

关联函数通常还用于模块创建的命名空间(以后再讲)

# PART5. 多个impl块

每个struct允许拥有多个impl块,这样可以将方法分组

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let square = Rectangle::square(3);
    println!("{:#?}", square.width);
    println!("{:#?}", square.height);
}
```