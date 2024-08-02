use std::fmt::Display;

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

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait bound语法的多个Trait bound(Trait约束)
pub fn notify<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait语法的多个Trait bound(Trait约束)
pub fn notify2(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}