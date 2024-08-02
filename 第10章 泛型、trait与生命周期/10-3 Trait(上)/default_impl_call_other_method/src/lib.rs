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