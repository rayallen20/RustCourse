/// 本Trait用于定义GUI元素的共有行为
pub trait Draw {
    fn draw(&self);
}

/// 本结构体用于表示屏幕 存储所有存在于屏幕中的GUI元素
/// 相比于使用泛型约束 使用特征对象的优点在于可以存储不同类型的GUI元素
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// 本方法用于在屏幕上绘制所有GUI元素
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}