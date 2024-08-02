# PART1. Drop Trait

- 实现Drop Trait,可以让我们自定义**当值将要离开作用域时发生的动作**
  - 例如: 文件、网络资源的释放等
  - 任何类型都可以实现Drop Trait
- Drop Trait只要求你实现`drop()`方法
  - 参数: 对self的可变引用(`&mut self`)
- Drop Trait在预导入模块中(prelude)中
  - 无需手动导入

Drop Trait的方法签名如下:

```rust
pub trait Drop {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn drop(&mut self);
}
```

例:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    /// `drop()`方法通常用于释放资源
    /// 此处出于演示的目的,我们只是打印了一句话
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // 创建变量时是先创建的c再创建的d
    // 而在main函数结束时,变量的释放顺序是先释放d再释放c
}
```

```
cargo run
   Compiling drop_trait_example v0.1.0 (/drop_trait_example)
...
warning: `drop_trait_example` (bin "drop_trait_example") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.47s
     Running `target/debug/drop_trait_example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

# PART2. 使用`std::mem::drop`来提前drop值

- 很难直接禁用自动的drop功能,而且也没有必要这么做
  - 因为Drop trait的目的就是进行**自动**释放处理
- Rust也不允许手动调用`Drop` trait的`drop()`方法

```rust
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };

  // Rust不允许手动调用`drop()`方法,因为这可能导致双重释放
  c.drop();

  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };

  println!("CustomSmartPointers created.");
}
```

```
cargo run
   Compiling explicit_call_drop_method v0.1.0 (/explicit_call_drop_method)
error[E0040]: explicit use of destructor method
  --> src/main.rs:17:7
   |
17 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
17 |     drop(c);
   |     +++++ ~

For more information about this error, try `rustc --explain E0040`.
error: could not compile `explicit_call_drop_method` (bin "explicit_call_drop_method") due to 1 previous error
```

- error[E0040]: explicit use of destructor method: 显式地使用了析构方法
- help: consider using `drop` function: 考虑使用`drop()`函数

调用`std::mem::drop`函数,就相当于提前调用了`Drop.drop()`方法

`std::mem::drop`函数是在预导入模块中的,因此无需手动导入

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // 手动调用std::mem::drop函数
    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}
```

```
cargo run
   Compiling call_drop_fn v0.1.0 (/call_drop_fn)
...

warning: `call_drop_fn` (bin "call_drop_fn") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/call_drop_fn`
Dropping CustomSmartPointer with data `my stuff`!
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
```

可以看到,当手动释放了`c`时,`c`的`drop()`方法被调用了