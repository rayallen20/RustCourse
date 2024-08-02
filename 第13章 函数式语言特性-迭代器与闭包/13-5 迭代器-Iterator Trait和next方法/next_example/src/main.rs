fn main() {
    let v1 = vec![1, 2, 3];

    // 这里加了mut关键字 是因为next方法会改变迭代器的内部状态
    // 可以理解为 next方法消耗掉了迭代器中的一个元素
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}
