pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger {
        // 使用RefCell<T>类型使得该字段可以在不可变的self上进行可变的操作
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // send_messages字段的类型为RefCell<Vec<String>> 而不是Vec<String>
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // RefCell<T>.borrow_mut()返回RefMut<T>类型,该类型它实现了DerefMut trait,所以可以直接使用*操作符
            // RefMut<T>智能指针可以修改其内部的值
            // 但是RefCell<T>类型同样会进行借用规则检查,只不过这个检查的过程延后到运行时了:
            // 每次调用RefCell<T>.borrow_mut()方法时,RefCell<T>会检查是否有其他代码正在使用RefCell<T>的不可变引用
            // 如果有,RefCell<T>.borrow_mut()方法会panic!
            // 可以有多个不可变引用,但是只能有一个可变引用 (这个规则是符合Rust的借用规则的)
            // 因此,调用RefCell<T>.borrow_mut()方法时,RefCell<T>同样会检查是否有其他代码正在使用RefCell<T>的不可变引用
            // 如果有,RefCell<T>.borrow_mut()方法也会panic!
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // RefCell<T>.borrow()方法返回Ref<T>类型,该类型实现了Deref trait,所以可以直接使用*操作符
        // Ref<T>也是一种智能指针,通过该指针可以访问RefCell<T>内部的值
        // 但是,Ref<T>类型只能用于访问内部值,不能用于修改内部值
        // 调用RefCell<T>.borrow()方法时同样会进行借用规则检查,
        // 若调用该方法时,存在RefCell<T>的可变引用,则会panic!
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        // 总结:
        // RefCell<T>在运行时强制执行以下借用规则:
        // 可以有多个不可变引用(通过RefCell<T>.borrow()获取)
        // 或者有一个可变引用(通过RefCell<T>.borrow_mut()获取)
        // 当存在可变借用时,不允许存在任何不可变借用
        // 当存在不可变借用时,不允许存在任何可变借用
    }
}
