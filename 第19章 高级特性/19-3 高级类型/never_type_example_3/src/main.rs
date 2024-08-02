impl <T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            // Some arm返回的类型为T
            Some(val) => val,
            // None arm返回的类型为 panic!()宏的返回值类型 即never类型
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}