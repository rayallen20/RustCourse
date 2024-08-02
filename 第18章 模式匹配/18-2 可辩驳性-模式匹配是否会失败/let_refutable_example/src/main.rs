fn main() {
    let a: Option<i32> = Some(5);

    // Some(x)是一个可辨驳的模式 因为无法匹配None变体
    // 而 let 语句要求模式是不可辨驳的
    let Some(x) = a; // error: refutable pattern in local binding: `None` not covered
}
