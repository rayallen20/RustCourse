# PART1. 生命周期标注说明

- **生命周期的标注,并不会改变引用的生命周期长度.它只是为了让借用检查器能够理解引用之间的关系**.
- 当某个函数指定了泛型生命周期参数,那么这个函数就可以接收任何生命周期的引用,而不是只能接收一种生命周期的引用.
- 生命周期的标注仅仅是**描述了多个引用之间的生命周期关系,并不会影响引用的生命周期**.
  - 你可以将它理解为是一种"声明",至于具体你的引用是否符合这个声明,则由编译器来判断.
  - 或者再举一个贴近一些的例子.例如函数签名中的参数类型,它仅仅是一个声明,声明调用该函数时,参数的类型应该是什么.至于实际调用时传入的参数是否符合这个声明,则由编译器来判断.

# PART2. 生命周期标注的语法

- 生命周期参数名:
  - 必须以`'`开头,后跟标识符
  - 要求标识符必须全小写,通常都会使用单个字母来表示生命周期参数
  - 很多人使用`'a`来表示生命周期参数

- 生命周期标注的位置:
  - 在引用符号`&`后
  - 使用空格将标注和引用类型分开

## 2.1 生命周期标注的例子

- `&i32`: 一个`i32`类型的引用
- `&'a i32`: 一个`i32`类型的引用,并且这个引用的生命周期是`'a`
- `&'a mut i32`: 一个`i32`类型的可变引用,并且这个引用的生命周期是`'a`

再次强调:单个生命周期标注本身没有意义.因为生命周期标注之所以存在,是为了向编译器描述多个引用之间的生命周期关系.

还回到上一节课的例子:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- `x: &'a str, y: &'a str`:声明x和y这2个引用要求和泛型生命周期`'a`存活一样的生命周期

# PART3. 函数签名中的生命周期标注

- 泛型生命周期参数声明在函数名和参数列表之间的`<>`中
  - 例:`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
  - 声明参数x、y和返回值具有相同的生命周期,而这个生命周期是`'a`
  - 这个函数签名告诉编译器这样一件事:
    - 有一个生命周期`'a`
    - 且函数有x和y这2个引用类型的参数
    - x和y这2个引用的存活时长不得短于生命周期`'a`
    - 函数的返回值也是一个引用类型
    - 这个引用的存活时长也不得短于生命周期`'a`

**这里再次强调:生命周期标注并不会改变引用的生命周期长度.它只是为了让我们向借用检查器指出一些约束,这些约束可以用于检查非法调用**.

这也就是为什么上节课中说,函数`longest()`并不需要知道x和y具体的存活时长

函数`longest()`只需要有一个生命周期`'a`,而这个生命周期能够满足函数签名中的约束即可.

实际上函数引用外部的代码,或者函数被外部的代码引用时,仅靠Rust编译器来确定引用的生命周期,这几乎是不可能的.

因为函数使用的生命周期可能在每次调用时都发生变化,这是编译器无法预知的.也正是因为这样,才需要我们手动进行生命周期标注.

再来看这段代码:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

问题来了:在这段代码中,`'a`所代表的生命周期究竟是哪里?

`'a`所代表的生命周期是x的作用域和y的作用域的交集,即两个作用域重叠的部分.

或者换句话说,`'a`所代表的生命周期是x和y的生命周期中最短的那个.

且由于返回值的生命周期也被标注为了`'a`,所以返回值也仅在x和y的生命周期重叠的部分内有效.

# PART4. 生命周期分析实例

将以上代码修改为:

```rust
fn main() {
  let string1 = String::from("abcd");                               // ----------+ string1的生命周期开始
  {                                                                 //           |
    let string2 = "xyz";                                            // ----------+ string2是一个字符串字面值,它的生命周期从引用被创建开始,到程序结束为止,一直有效
    let result = longest(string1.as_str(), string2);                // ----------+ string1.as_str()的生命周期开始. Tips: string1.as_str()的生命周期结束时间与string1的生命周期结束时间相同.因为as_str()方法返回的是对string1的引用
    println!("The longest string is {}", result);                   //           |
  }                                                                 // ----------+
}                                                                   // ----------+ string1的生命周期结束

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

在这段代码中:

- `string1`的生命周期从它被创建开始,到它被销毁为止.也就是从第2行开始,到第7行(包含第7行)为止;
- `string2`是一个字符串字面值,它的生命周期从引用被创建开始,到程序结束为止;
  - 也就是说`string2`的生命周期从第4行开始,到第7行(包含第7行)为止;
- `string1.as_str()`的生命周期与`string1`的生命周期相同.因为`as_str()`方法返回的是对`string1`的引用;
  - 也就是说`string1.as_str()`的生命周期从第5行开始,到第7行(包含第7行)为止;
- 由此可以得出,`'a`所代表的生命周期是`string1.as_str()`和`string2`的生命周期的交集,即从第5行开始,到第7行(包含第7行)为止.
- 由于返回值的生命周期也被标注为了`'a`,所以返回值`result`的引用也仅在`string1.as_str()`和`string2`的生命周期重叠的部分内有效.
  - 但是要注意作用域,变量`result`的作用域是从第4行开始,到第6行(包含第6行)为止.

结论:

- `'a`指代的生命周期从从第5行开始,到第7行(包含第7行)为止
- `result`的生命周期从从第5行开始,到第7行(包含第7行)为止
- 但是`result`的作用域是从第4行开始,到第6行(包含第6行)为止
- 这里不要把生命周期和作用域混淆了,生命周期是指引用的有效时间,而作用域是指变量的有效范围

再将本例做如下变更:

```rust
fn main() {
    let string1 = String::from("abcd");                                     // ----------+ string1的生命周期开始
    let result;                                                                 // ----------+ result有效范围开始(注意是有效范围而不是生命周期)
    {                                                                                 //           |
        let string2 = String::from("xyz");                                  // ----------+ string2的生命周期开始
        result = longest(string1.as_str(), string2.as_str());                         // ----------+ string2.as_str()的生命周期开始
    }                                                                                 // ----------+ string2的生命周期结束 这意味着string2.as_str()的生命周期也同时结束
    println!("The longest string is {}", result);                                     // ----------+ result有效范围结束
}                                                                                     // ----------+ string1的生命周期结束

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

在这段代码中:

- `'a`指代的生命周期是`string2`的生命周期(因为`string2`的生命周期比`string1`的生命周期短),即从第5行开始,到第7行(包含第7行)为止

```
lifecycle_example_2 % cargo run  
   Compiling lifecycle_example_2 v0.1.0 (/lifecycle_example_2)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
5 |         let string2 = String::from("xyz");                                  // ----------+ string2的生命周期开始
  |             ------- binding `string2` declared here
6 |         result = longest(string1.as_str(), string2.as_str());                         // ----------+ string2.as_str()的生命周期开始
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }                                                                                 // ----------+ string2的生命周期结束 这意味着string2.as_str()的生命周期也同时结束
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);                                     // ----------+ result有效范围结束
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifecycle_example_2` (bin "lifecycle_example_2") due to 1 previous error
```

这段代码编译失败,因为`string2`的生命周期比`result`的生命周期短,导致`result`引用的生命周期超出了`string2`的生命周期.