fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);

    // 此处*的含义为解引用,即取出y指向的值
    assert_eq!(5, *y);
}
