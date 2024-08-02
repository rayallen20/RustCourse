# PART1. 生命周期

Rust中,每个引用都有自己的生命周期

生命周期: 让引用保持有效的作用域

大多数情况下,生命周期是隐式的、可以被推断的

当引用的生命周期以不同的方式互相关联时,需要显式标注生命周期

# PART2. 生命周期的作用

生命周期的作用:避免悬垂引用(dangling reference)

```rust
fn main() {
    {
        // 变量r此时只是被声明了,没有被初始化
        // 这么做的目的是为了让r存在于x的作用域之外
        // Rust中,不允许空值存在.但是r此时还没有被初始化,因此r并不是一个空值
        let r;

        // 如果此时使用了r 则会报错
        // let b = r; // 这里会报错,因为r没有被初始化 (error[E0381]: borrow of possibly-uninitialized variable: `r`)

        {
            let x = 5;
            r = &x; // error[E0597]: `x` does not live long enough
        }

        // 在此处使用r时会报错 因为r指向的值(也就是x的引用)已经被释放了
        // 因为x已经离开了自己的作用域 所以x已经被释放了 进而x的引用也就失效了
        // 所以此时r指向的是一个已经被释放的内存 也就是悬垂引用
        println!("r: {}", r);
    }
}
```

```
cargo build
   Compiling lifecycle_example v0.1.0 (/lifecycle_example)
error[E0597]: `x` does not live long enough
  --> src/main.rs:13:17
   |
12 |             let x = 5;
   |                 - binding `x` declared here
13 |             r = &x; // error[E0597]: `x` does not live long enough
   |                 ^^ borrowed value does not live long enough
14 |         }
   |         - `x` dropped here while still borrowed
...
19 |         println!("r: {}", r);
   |                           - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifecycle_example` (bin "lifecycle_example") due to 1 previous error
```

# PART3. 借用检查器

Rust编译器的借用检查器:比较作用域,进而判断代码中所有的引用是否合法

```rust
fn main() {
    {
        let r;                            // ---------+-- 'a  r的生命周期开始
        {                                 //          |
            let x = 5;                    // -+-- 'b  |       x的生命周期开始
            r = &x;                       //  |       |
        }                                 // -+       |       x的生命周期结束
                                          //          |
                                          //          |
        println!("r: {}", r);             //          |
    }                                     // ---------+       r的生命周期结束
}
```

在编译的过程中,编译器会比较r和x的生命周期,编译器发现r的生命周期比x的生命周期要长,因此r指向的x的引用是不合法的.因此编译没有通过.

怎么解决这个问题呢? 很明显,让生命周期'b的长度不小于生命周期'a的长度即可:

```rust
fn main() {
    let x = 5;                       // ----------+-- 'b
    let r = &x;                      // --+--      |  'a
    println!("r: {}", r);            // -----------+
}
```

这样做,在生命周期'a内,x是一直有效的.换言之,声明周期'a的长度小于等于生命周期'b的长度,因此r指向的x的引用是合法的.

# PART4. 函数中的泛型生命周期

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这段代码是无法通过编译的:

```
cargo build
   Compiling genericity_lifecycle_in_fn v0.1.0 (/genericity_lifecycle_in_fn)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

error: lifetime may not live long enough
  --> src/main.rs:11:9
   |
9  | fn longest(x: &str, y: &str) -> &str {
   |               - let's call the lifetime of this reference `'1`
10 |     if x.len() > y.len() {
11 |         x
   |         ^ returning this value requires that `'1` must outlive `'static`

error: lifetime may not live long enough
  --> src/main.rs:13:9
   |
9  | fn longest(x: &str, y: &str) -> &str {
   |                        - let's call the lifetime of this reference `'2`
...
13 |         y
   |         ^ returning this value requires that `'2` must outlive `'static`

For more information about this error, try `rustc --explain E0106`.
error: could not compile `genericity_lifecycle_in_fn` (bin "genericity_lifecycle_in_fn") due to 3 previous errors
```

错误出现的地方为:

```
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
```

help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`: 函数的返回类型包含一个借用值,但是**函数签名**(注意是函数签名)中没有说明这个借用值是从`x`还是`y`中借用的

```
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

这里的意思是: 考虑引入一个命名的生命周期参数

这里先确定一件事:这个报错与函数最终究竟返回x的借用还是y的借用无关.

修改代码如下:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    x
}
```

```
cargo build
   Compiling genericity_lifecycle_in_fn v0.1.0 (/genericity_lifecycle_in_fn)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

warning: unused variable: `y`
 --> src/main.rs:9:21
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |                     ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

error: lifetime may not live long enough
  --> src/main.rs:10:5
   |
9  | fn longest(x: &str, y: &str) -> &str {
   |               - let's call the lifetime of this reference `'1`
10 |     x
   |     ^ returning this value requires that `'1` must outlive `'static`

For more information about this error, try `rustc --explain E0106`.
warning: `genericity_lifecycle_in_fn` (bin "genericity_lifecycle_in_fn") generated 1 warning
error: could not compile `genericity_lifecycle_in_fn` (bin "genericity_lifecycle_in_fn") due to 2 previous errors; 1 warning emitted
```

报错信息和修改前是一样的:

```
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

Tips: 如果将函数签名改为`fn longest(x: &str) -> &str`,是可以通过的.因为此时可以通过函数签名来确定返回值是从`x`中借用的.

换句话说,这里报错的核心原因在于:函数的返回值是`x`和`y`中的某一个的借用,但是函数签名中没有指明这个借用值是从`x`还是`y`中借用的.

即使函数的返回值是`x`的借用,也是一样的.**因为编译器不会检查函数的实现,只会检查函数的签名**.或者你也可以理解为,编译器不会检查你的逻辑是否正确,只会检查你的签名是否正确.

而且由于x和y处于形参的位置上,所以也没有办法像之前一样,通过比较作用域的方式来确定生命周期.也就是说,借用检查器也无法确定x和y的生命周期是否合法.

本例中,x和y这两个参数的生命周期,对于函数`longest()`来说,是不确定的.

从函数`longest()`的视角出发,x和y的类型为`&str`,也就是说,x和y都是引用.而这两个引用的声明周期,对于函数`longest()`来说,是不确定的.

且该函数的返回值也是一个引用,但该函数无法确认的一件事是:返回值的生命周期是如何与x和y的生命周期相关联的?(或者说返回值的生命周期与x和y的生命周期有什么关系?)

因此需要**显式标注生命周期**,用以关联返回值的生命周期与x和y的生命周期.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 'a: 生命周期参数.表示有一个名为'a的生命周期
// x: &'a str : 表示x是一个字符串切片，且其生命周期至少与'a一样长
// y: &'a str : 表示y是一个字符串切片，且其生命周期至少与'a一样长
// -> &'a str : 表示返回值的生命周期至少与'a一样长
// 此时,返回值、x、y的生命周期是相同的(这个说法不太准确)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```
cargo run
   Compiling genericity_lifecycle_in_fn v0.1.0 (/genericity_lifecycle_in_fn)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/genericity_lifecycle_in_fn`
The longest string is abcd
```

TODO: 这里我不明白的是,为什么函数返回值的生命周期要与参数的生命周期有关联?