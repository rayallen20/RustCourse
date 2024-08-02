use std::fmt::{Debug, Display};

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

// 可以看到 当有2个泛型参数,且每个泛型参数有2个trait bound时,函数名和形参列表之间的Trait Bound就已经很长了
pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

// 使用where关键字可以使代码更加简洁
pub fn notify_where<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}
