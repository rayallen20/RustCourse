fn main() {
    let s = String::from("这是中文字符串");
    // let slice1: &str = &s[0..2];
    // println!("{}", slice1);

    // let slice2: &str = &s[0..3];
    // println!("{}", slice2);

    let slice3: &str = &s[..];
    for c in slice3.chars() {
        println!("{}", c);
    }
}
