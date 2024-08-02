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