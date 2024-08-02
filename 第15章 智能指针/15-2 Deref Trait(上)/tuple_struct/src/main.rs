struct Color(i32, i32, i32);

impl Color {
    fn describe(&self) {
        // 元组结构体的字段通过索引位置访问
        println!("RGB({}, {}, {})", self.0, self.1, self.2);
    }
}

fn main() {
    let color = Color(255, 165, 0);
    color.describe();
}
