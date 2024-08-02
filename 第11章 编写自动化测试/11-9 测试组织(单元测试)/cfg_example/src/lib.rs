#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    // 这个函数也会在测试时被编译
    fn nothing() {

    }
}
