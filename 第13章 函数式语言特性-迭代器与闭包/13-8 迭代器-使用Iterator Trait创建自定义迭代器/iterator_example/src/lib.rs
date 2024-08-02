struct Counter {
    // 本字段用于存储迭代过程中的数值(也可以认为是状态)
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 指定关联类型为u32 这个语法后边再讲
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let c1_iterator = Counter::new();
        let c2_iterator = Counter::new().skip(1);
        let c3_iterator = c1_iterator.zip(c2_iterator);

        let multi_closure = |a, b| a * b;
        let c4_iterator = c3_iterator.map(|(a, b)| multi_closure(a, b));

        let filter_closure = |x| x %3 == 0;
        let result_iterator = c4_iterator.filter(|x| filter_closure(*x));
        let result: u32 = result_iterator.sum();
        assert_eq!(result, 18);
    }

    #[test]
    // 上面的代码可以简化为下面的代码
    fn using_other_iterator_trait_methods1() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}