pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(add_two(100), 102);
    }
}
