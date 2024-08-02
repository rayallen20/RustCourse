use std::ops::Add;

/// 本类型表示毫米
struct Millimeters(i32);

// 指定rhs的类型默认为Meters
impl Add<Meters> for Millimeters {
    // 指定关联类型Output为Millimeters
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

/// 本类型表示米
struct Meters(i32);

fn main() {
    let millimeters = Millimeters(1000);
    let meters = Meters(1);
    let result = millimeters + meters;
    assert_eq!(result.0, 2000);
}
