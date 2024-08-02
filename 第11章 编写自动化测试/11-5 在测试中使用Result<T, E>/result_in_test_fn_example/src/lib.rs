#[cfg(test)]
mod tests {
    #[test]
    // 此处让测试函数返回Result即可
    fn two_plus() ->Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
