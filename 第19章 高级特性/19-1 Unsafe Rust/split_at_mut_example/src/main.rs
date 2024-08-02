fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);
}


fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = slice.len();

    // 判断用于切割的索引是否合法
    assert!(mid <= len);

    // 此处对slice进行了两次可变引用,这是借用检查器不允许的
    (&mut slice[..mid], &mut slice[mid..])  // error: cannot borrow `*slice` as mutable more than once at a time
}