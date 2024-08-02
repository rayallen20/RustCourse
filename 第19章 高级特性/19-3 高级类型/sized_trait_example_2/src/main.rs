// ?表示不确定性 表示泛型T可能是Sized也可能不是Sized
// 但是?只能用在Sized trait上,不能用于其他trait
// 另外,由于T的大小可能是不确定的,因此需要把类型T放在某种指针之后,比如&T或者Box<T>
fn generic<T: ?Sized> (t: &T) {
    // do something
}

fn main() {}
