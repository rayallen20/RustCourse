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

// Trait Bound写法
// 在比较复杂的情况下, Trait Bound写法更加简洁
// 注意: 在调用notify时, item1和item2的类型必须相同
// 这里的类型相同,不是指item1和item2均为Summary Trait的实现,而是指它们是同样的具型
pub fn notify<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

// impl Trait参数的写法 比Trait Bound写法的方法签名要长
pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}