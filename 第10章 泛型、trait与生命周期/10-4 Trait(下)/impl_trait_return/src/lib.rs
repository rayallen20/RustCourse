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

// 注意: 这里虽然函数签名上写的返回类型为Summary Trait的实现即可
// 但是要求返回值的类型必须是确定的具型,不能是动态的
// 这一点和使用Trait作为参数是相同的.我猜测也是因为单态化的原因
pub fn notify(s: &str) -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(s),
        reply: false,
        retweet: false,
    }
}

// 这里的返回值类型是动态的,所以编译器会报错
// error[E0308]: `if` and `else` have incompatible types
// pub fn notify1(s: &str) -> impl Summary {
//     if s.len() > 10 {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(s),
//             reply: false,
//             retweet: false,
//         }
//     }
// }