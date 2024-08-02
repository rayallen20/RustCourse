fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| {z == x};

    // 此处已经不能再访问x了 因为x的所有权已经被闭包消耗掉了
    println!("can't use x here: {:?}", x);  // error: value borrowed here after move

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
