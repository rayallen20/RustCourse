fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 'a: 生命周期参数.表示有一个名为'a的生命周期
// x: &'a str : 表示x是一个字符串切片，且其生命周期至少与'a一样长
// y: &'a str : 表示y是一个字符串切片，且其生命周期至少与'a一样长
// -> &'a str : 表示返回值的生命周期至少与'a一样长
// 此时,返回值、x、y的生命周期是相同的(这个说法不太准确)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
