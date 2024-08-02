# PART1. 方法定义中的生命周期标注

在Struct上使用生命周期实现方法,语法和泛型参数的语法是相同的

在何处声明和使用生命周期参数,依赖于:

- 生命周期参数是否和字段、方法的参数、返回值有关联

struct字段的生命周期名:

- 在impl关键字后面声明生命周期参数
- 在struct名后使用
- 因为这些生命周期是struct类型的一部分

在impl块内的方法签名中:

- 引用要么是绑定于struct字段引用的生命周期,要么是独立的生命周期
- 生命周期省略规则经常使得方法中的生命周期标注不是必须的

例:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// impl<'a> 结构体字段的生命周期参数,需要声明在impl关键字后边
// ImportantExcerpt<'a> 表示这个生命周期应用于ImportantExcerpt结构体 注意: 这里的生命周期参数是ImportantExcerpt类型声明的一部分
impl<'a> ImportantExcerpt<'a> {
    // &self可以不标注生命周期 因为它的生命周期与ImportantExcerpt结构体的生命周期一样
    fn level(&self) -> i32 {
        3
    }

    // 根据规则1: fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &str
    // 该函数不适用规则2
    // 根据规则3: fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
    // 至此,所有引用的生命周期参数都确定了
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("ImportantExcerpt: {}", i.part);
}
```

# PART2. 静态生命周期

- `'static`是一个特殊的生命周期: 整个程序运行期间都有效

所有的字符串字面量都拥有`'static`生命周期:

- `let s: &'static str = "I have a static lifetime.";`

之间讲过,字符串字面量是被存储在二进制文件中的,所以它们总是可用的.因此,所有字符串字面量都是`'static`生命周期

为引用指定`'static`生命周期前要三思:

- 是否需要引用在程序的整个生命周期内都存活

大部分情况下,错误的原因都在于尝试创建一个悬垂引用或生命周期不匹配.这种情况下应该尝试解决问题,而非使用`'static`生命周期来掩盖问题

# PART3. 泛型参数类型、Trait Bound、生命周期混合的例子

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
```