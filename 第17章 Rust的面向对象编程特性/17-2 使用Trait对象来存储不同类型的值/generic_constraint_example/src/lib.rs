/// 本Trait用于定义GUI元素的共有行为
pub trait Draw {
    fn draw(&self);
}

/// 本结构体用于表示屏幕 存储所有存在于屏幕中的GUI元素
/// 使用泛型约束则要求传入的T的具型是相同的 你无法将Button和TextField同时存储在Screen中
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw
{
    /// 本方法用于在屏幕上绘制所有GUI元素
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}