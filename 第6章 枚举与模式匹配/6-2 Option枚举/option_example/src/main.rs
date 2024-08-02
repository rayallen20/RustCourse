fn main() {
    // some_number的类型为 Option<i32> 而非i32
    let some_number = Some(5);

    // some_string的类型为 Option<&str> 而非&str
    let some_string = Some("a string");

    // 无法通过None变体来推断出absent_number的类型 因此需要显式指定类型
    let absent_number: Option<i32> = None;
}
