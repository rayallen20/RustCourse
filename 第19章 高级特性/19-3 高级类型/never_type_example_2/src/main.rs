fn main() {
    let guess = "";

    loop {
        // match的各个arm的返回值类型必项一致
        // Ok arm返回的是u32类型
        // 而Err arm返回的是continue
        // continue返回的就是一个 never 类型 (!类型)
        // 但never类型无法产生一个可供返回的值
        // 因此这个match表达式的值就采用了Ok arm的返回值类型 即u32类型
        // 而never类型则被强制转换为了其他任意类型 (never类型的表达式可以被强制转换为任意其他类型)
        // 因此这2个arm返回的类型是一致的
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}