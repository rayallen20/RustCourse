fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {   // 匹配Option<T>的Some(T)值 如果favorite_color是Some(T)变体就执行{}里的代码 如果favorite_color是None变体就跳过这个代码块
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {   // 匹配Result<T, E>的Ok(T)值 如果age是Ok(T)变体就执行{}里的代码 如果age是Err(E)变体就跳过这个代码块
        // 注意: 这个代码块中的age是u8类型的值 不是作用域外的age变量 换言之 这里的age不是外边的Result<u8, _>类型
        if age > 30 {
            println!("Using purple as the background color");   // 最终会打印这句话
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
