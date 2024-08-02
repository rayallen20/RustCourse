#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height

        // 模拟一个错误的实现
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // tests模块只是一个普通的模块 在该模块中想要使用其他模块的内容
    // 同样需要先导入 此处导入的是lib.rs中的所有公有项
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // Arrange: 准备数据阶段
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        // Act: 执行代码阶段
        let result = larger.can_hold(&smaller);

        // Assert: 验证结果阶段
        assert!(result);
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        let result = smaller.can_hold(&larger);

        // 此处result的值应该是false 加了取反 断言结果为true
        assert!(!result);
    }
}
