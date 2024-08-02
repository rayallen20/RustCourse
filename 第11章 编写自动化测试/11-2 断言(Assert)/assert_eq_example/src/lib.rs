pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // Rust中,期待的值和执行结果,放在具体哪个位置上都可以
        // 但在一些其他语言中,可能会对位置有要求
        assert_eq!(4, add_two(2));
    }
}
