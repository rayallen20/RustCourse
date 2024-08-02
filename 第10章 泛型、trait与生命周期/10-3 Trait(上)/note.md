# PART1. 简介

Trait告诉Rust编译器:

- 某种特定类型具有哪些功能,且这个类型可以与其他类型共享这些功能

Trait以抽象的方式来定义共享的行为

Trait bounds(约束): 将泛型类型参数指定为实现了某个特定行为(即特定trait)的类型.换言之,就是要求泛型的类型参数实现了特定的trait

Trait与其他语言的接口(interface)类似,但是也有区别

# PART2. 定义Trait

一个类型的行为由该类型可调用的方法构成.但是如果不同的类型有相同的方法,那么可以说这些类型共享了相同的行为.这时就可以使用Trait来定义这些共享的行为

Trait的定义: 把方法签名放在一起,来定义实现某种目的所必须的一组行为

- Trait中,只有方法签名,没有方法实现(这句话不绝对,后面会讲到)
- Trait中可以有多个方法签名
- 实现该Trait的类型,必须实现Trait中的所有方法签名

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
// 本trait用于定义摘要行为
// 注意: 本trait是一个公共trait 这意味着在lib.rs(library crate的根模块)中定义的该Trait可以被其他crate使用
pub trait Summary {
    // 本方法用于生成一个摘要内容
    fn summarize(&self) -> String;
}
```

# PART3. 在类型上实现Trait

实现Trait的语法: 

- `impl TraitName for TypeName {...}`
- 在impl块中,实现Trait中的方法签名(这句话不绝对,后面会讲到)

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
    fn summarize(&self) -> String;
}

// 新闻文章结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 新闻文章实现Summary特性
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 推特文章结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 推特文章实现Summary特性
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

main.rs:

```rust
// 注意: 来自Trait中的方法,只有当Trait在当前作用域中才能调用
use impl_trait::Summary;
use impl_trait::NewsArticle;

fn main() {
    let news = NewsArticle {
        headline: String::from("新闻标题"),
        location: String::from("中国"),
        author: String::from("张三"),
        content: String::from("新闻内容"),
    };

    // 调用NewsArticle结构体的summarize()方法
    // 注意: summarize()方法是定义在Summary Trait上的 而不是定义在NewsArticle结构体上的
    // 因此要调用这个方法, 就需要引入Summary Trait
    // 这也是为什么在lib.rs中将Summary Trait定义为pub的原因
    println!("新闻摘要: {}", news.summarize());

    let tweet = impl_trait::Tweet {
        username: String::from("张三"),
        content: String::from("推特内容"),
        reply: false,
        retweet: false,
    };
    println!("推特摘要: {}", tweet.summarize());
}
```

运行结果:

```
 cargo run
   Compiling impl_trait v0.1.0 (/impl_trait)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/impl_trait`
新闻摘要: 新闻标题, by 张三 (中国)
推特摘要: 张三: 推特内容
```

**注意:来自Trait中的方法,只有当Trait在当前作用域中才能调用**

# PART4. 实现Trait的约束

可以在某个类型上实现某个Trait的前提条件:

- 该类型或该Trait必须是在本地crate中定义的
  - 例: `Tweet`类型可以在`lib.rs`(library crate)中实现`std::fmt::Display` Trait
  - 例: 在`lib.rs`(library crate)中定义的Trait `impl_trait::Summary`可以被`Vec<T>`类型实现
- 无法为外部类型实现外部Trait
  - 例: 无法在`impl_trait` package中为`Vec<T>`类型实现`std::fmt::Display` Trait
  - 这个限制是程序属性的一部分(也就是**一致性**)
  - 更具体的说是孤儿规则:
    - 孤儿规则的定义是:只要Trait或类型对于当前crate是本地的,就可以在该类型上实现该Trait
    - 之所以叫做孤儿规则,是因为父类型(这里的父类型指的是你试图在该类型上实现Trait的类型)没有在当前crate中定义,所以父类型是"孤儿"
  - 孤儿规则确保了他人的代码不能破坏你的代码,反之亦然
  - 如果没有这个规则,那么2个crate就可以为同一个类型实现同一个Trait,这时就会出现冲突
    - 如果允许为外部类型实现外部Trait,那么就会出现这样的情况:外部某类型(我们称为TypeA)已经实现了某Trait(我们称为TraitA),而我们在我们的crate中也为TypeA实现了TraitA,那么这时就会出现冲突
      - 编译器将不知道该使用TraitA的哪个实现

# PART5. 默认实现

## 5.1 简介

Trait中的方法签名可以有默认实现,这样就可以无需让每个实现该Trait的类型都实现这个方法,仅针对某些特定类型重新实现这个方法即可

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
    // 默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 使用默认实现
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    // 重载默认实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

main.rs:

```rust
use default_impl::Summary;
use default_impl::NewsArticle;

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("{}", article.summarize());
}
```

运行结果:

```
cargo run  
   Compiling default_impl v0.1.0 (/default_impl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running `target/debug/default_impl`
(Read more...)
```

## 5.2 默认实现调用Trait中的其他方法

Trait中的默认实现可以调用Trait中的其他方法,即使这些方法没有实现

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
```

main.rs:

```rust
use default_impl_call_other_method::Summary;
use default_impl_call_other_method::NewsArticle;

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("{}", article.summarize());

    let tweet = default_impl_call_other_method::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}
```

运行结果:

```
cargo run
   Compiling default_impl_call_other_method v0.1.0 (/default_impl_call_other_method)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/default_impl_call_other_method`
(Read more from @Iceburgh...)
horse_ebooks: of course, as you probably already know, people
```

**注意:无法在方法的重载实现中,调用该方法的默认实现**