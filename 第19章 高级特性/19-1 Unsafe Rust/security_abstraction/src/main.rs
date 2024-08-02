fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(b, &[4, 5, 6]);
}
