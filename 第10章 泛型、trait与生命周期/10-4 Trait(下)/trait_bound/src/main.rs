use trait_bound::Tweet;
use trait_bound::notify;

fn main() {
    let tweet1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // 调用notify()函数时, item1和item2的类型必须相同
    // 我猜测这和编译时的单态化有关
    notify(tweet1, tweet2);
}
