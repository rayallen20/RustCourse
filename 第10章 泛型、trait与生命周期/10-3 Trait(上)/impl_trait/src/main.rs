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
