fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引访问 越界会导致panic
    let third: &i32 = &v[100];
}
