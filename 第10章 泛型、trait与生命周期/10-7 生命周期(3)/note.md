# PART1. 深入理解生命周期

- 指定生命周期参数的方式,依赖于函数所做的事情

现有某函数如下:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

如果现在该函数确定返回x,那么可以省略y的生命周期参数,如下:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

- 从函数返回引用时,返回值的生命周期参数需要与其中一个入参的生命周期参数匹配
- 如果返回的引用没有指向任何一个入参,那么这个引用只能是引用了函数内部创建的一个值
  - 这个值会在函数结束时走出作用域,也就是说发生了悬垂引用

例:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("abc");
    result.as_str() // error: cannot return value referencing local variable `result`
}
```

```
cargo run
   Compiling lifecycle_example_4 v0.1.0 (/lifecycle_example_4)
...

error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:10:5
   |
10 |     result.as_str() // error:
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
warning: `lifecycle_example_4` (bin "lifecycle_example_4") generated 2 warnings
error: could not compile `lifecycle_example_4` (bin "lifecycle_example_4") due to 1 previous error; 2 warnings emitted
```

那么问题来了:如果想把函数内部创建的值返回出去,该怎么办呢?

答案其实很简单:直接把这个值返回出去

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> String {
    let result = String::from("abc");
    result
}
```

这种方式相当于把`result`持有的数据的所有权转移给了调用者,这样就不会出现悬垂引用的问题了

从根本上讲,生命周期参数是用于关联函数的不同参数与返回值之间的生命周期的.一旦返回值与参数之间产生了某种联系,Rust借用检查器就获得了足够的信息,用以支持保证内存安全的操作,并阻止可能会导致悬垂引用或其他违反内存安全的行为.

# PART2. Struct定义中的生命周期标注

Struct中可以包括:

- 自持有的类型(i32/bool/String等)
- 引用类型:需要在每个引用上都标注生命周期参数

例:

```rust
struct ImportantExcerpt<'a> {
    // 此处的'a是一个生命周期标注
    // 表示ImportantExcerpt的实例不能比其part字段的引用存在的更久
    // 或者换言之 part字段的引用要比ImportantExcerpt的实例存在的更久 且 要求part字段的引用的生命周期能够完全覆盖ImportantExcerpt的实例的生命周期
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,       // // 此处part字段的生命周期与first_sentence的生命周期相同 也就是第10行到第14行的生命周期 而ImportantExcerpt实例的生命周期是第11行到第14行
    };
    println!("{}", i.part);
}
```

# PART3. 生命周期的省略

我们知道:

- 每个引用都有生命周期
- 需要为使用生命周期的函数或struct指定生命周期参数

但是,之前我们写过这样一个函数:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

这个函数没有标注任何生命周期,但是仍然可以通过编译,这是因为Rust有一套自动推断生命周期的规则,这套规则被称为**生命周期省略规则**.其本质是在Rust引用分析中已经编入的一些常见模式,使得Rust编译器能够自动推断出生命周期参数的值.

注:在Rust的早期版本中,这段代码是无法通过编译的,因为Rust的生命周期检查器要求我们为每个引用都指定生命周期参数.但是随着Rust的发展,生命周期省略规则被引入,使得我们可以省略一些生命周期参数

在Rust引用分析中所编入的模式被称为生命周期省略规则:

- 这些规则无需开发者来遵守
- 它们是一些特殊情况,由编译器来考虑
- 如果你的代码符合这些情况,那么就无需显式标注生命周期参数

但是,生命周期省略规则是不会提供完整的推断的:

- 如果应用规则后,引用的生命周期仍然模糊不清,那么Rust编译器会报错
- 解决办法:添加生命周期标注,明确表明引用之间的相互关系

# PART4. 输入、输出生命周期

- 生命周期出现在函数/方法的参数中: 称为输入生命周期
- 生命周期出现在函数/方法的返回值中: 称为输出生命周期

# PART5. 生命周期省略的3个规则

编译器使用3个规则在没有显式标注生命周期的情况下,来确定引用的生命周期

- 规则1应用于输入生命周期
- 规则2、3应用于输出生命周期
- 如果编译器应用完这3个规则之后,仍然有无法确定生命周期的引用,则会报错
- 这些规则适用于fn定义和impl块

## 5.1 规则1:每个引用类型的参数都有自己的生命周期参数

可以理解为,单参数的函数,有1个生命周期参数;有2个参数的函数,则有2个不同的生命周期参数,以此类推

## 5.2 规则2:如果只有1个输入生命周期参数,那么该生命周期参数将会被赋给所有输出生命周期参数

## 5.3 如果有多个输入生命周期参数,但其中一个是&self或&mut self(即方法),那么self的生命周期参数会被赋给所有输出生命周期参数

# PART6. 生命周期省略规则的例子

假设我们自己是编译器,根据上面的规则,我们来推断下面的代码的生命周期:

## 6.1 `fn first_word(s: &str) -> &str`

- 应用规则1后: `fn first_word<'a>(s: &'a str) -> &str`
- 因为这个函数只有1个输入生命周期参数,所以规则2也适用.应用规则2后: `fn first_word<'a>(s: &'a str) -> &'a str`

到此,函数`first_word()`的入参与返回值的所有生命周期参数都已经确定了.可以继续推断后续的代码了.

## 6.2 `fn longest(x: &str, y: &str) -> &str`

- 应用规则1后: `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str`
- 这个函数不符合规则2的条件,所以不适用规则2
- 这个函数也不符合规则3的条件,所以不适用规则3

应用完3条规则之后,函数`longest()`的返回值的生命周期参数仍然无法确定,需要开发者手动标注