fn main() {
    let string1 = String::from("abcd");                               // ----------+ string1的生命周期开始
    {                                                                           //           |
        let string2 = "xyz";                                              // ----------+ string2是一个字符串字面值,它的生命周期从引用被创建开始,到程序结束为止,一直有效
        let result = longest(string1.as_str(), string2);                  // ----------+ string1.as_str()的生命周期开始. Tips: string1.as_str()的生命周期结束时间与string1的生命周期结束时间相同.因为as_str()方法返回的是对string1的引用
        println!("The longest string is {}", result);                           //           |
    }                                                                           // ----------+
}                                                                               // ----------+ string1的生命周期结束

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}