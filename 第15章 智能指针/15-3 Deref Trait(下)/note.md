# PART1. 函数和方法的隐式解引用转化(Deref Coercion)

- 隐式解引用转化(Deref Coercion)是为函数和方法提供的一种便捷特性
- 假设类型`T`实现了Deref Trait:
  - Deref Coercion可以把`T`的引用转化为`T`经过Deref操作后生成的引用

应用场景:

- 当把某类型的引用传递给函数或方法时,但它的类型与定义的参数类型不匹配:
  - 此时Deref Coercion就会自动发生(因此称为**隐式**解引用转化)
  - 编译器会对`deref()`方法进行**一系列**调用,直到把该引用转换为所需的参数类型未知
    - 这些操作在编译时就完成了,没有额外的性能开销

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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // &m: 其类型为&MyBox<String>
    // 由于MyBox类型实现了Deref trait, 所以Rust会自动调用deref方法, 将&MyBox<String>转换为&String
    // 由于String类型也实现了Deref trait, 所以Rust会再次自动调用deref方法, 将&String转换为&str (String类型的deref方法返回的类型为&str)
    // 这样 就满足hello()函数的参数类型要求了
    hello(&m);

    // 如果Rust没有Deref coercion, 那么上面的代码需要写成下面这样

    // step1. 获得一个&String类型的值
    let s = &(*m);
    // step2. 将&String类型的值转换为&str类型的值
    let str = &s[..];
    hello(str);
}
```

```
cargo run
   Compiling deref_coercion_example v0.1.0 (/deref_coercion_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/deref_coercion_example`
Hello, Rust!
Hello, Rust!
```

这就是上文说的,只要类型`T`实现了Deref Trait,那么Rust就会自动分析类型,当类型不匹配时,会不断尝试调用`deref()`方法,直到将`&T`转换为所需的参数类型为止

而且这个过程是在编译时完成的,不会有额外的性能开销

注: String类型对Deref trait的实现如下:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}
```

可以看到,String类型的`deref()`方法返回的类型为`&str`

# PART2. 解引用与可变性

- 可使用DerefMut Trait来重载可变引用的`*`运算符
- 在类型和trait在下列3种情况发生时,Rust会执行deref coercion:
  - 当`T: Deref<Target=U>`时,允许`&T`转换为`&U`
    - 类型`T`实现了Deref Trait,而它的`deref()`方法返回的类型为`U`,那么当Deref coercion发生时,`&T`会被转换为`&U`
  - 当`T: DerefMut<Target=U>`时,允许`&mut T`转换为`&mut U`
    - 类型`T`实现了DerefMut Trait,而它的`deref_mut()`方法返回的类型为`U`,那么当Deref coercion发生时,`&mut T`会被转换为`&mut U`
  - 当`T: Deref<Target=U>`时,允许`&mut T`转换为`&U`
    - 类型`T`实现了Deref Trait,而它的`deref()`方法返回的类型为`U`,那么当Deref coercion发生时,`&mut T`会被转换为`&U`
    - 这个场景下,Deref coercion会将一个可变引用(`&mut T`)转换为一个不可变引用(`&U`)
    - 但是反之不可以,Deref coercion不能将一个不可变引用(`&T`)转换为一个可变引用(`&mut U`),因为Rust要求可变引用是唯一的,但是Deref coercion是无法保证这一点的
    - 因为Deref coercion是无法得知在同一时刻是否还存在其他的可变引用指向了同一个数据