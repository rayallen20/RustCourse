fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // sum()方法会取得迭代器的所有权
    let total: i32 = v1_iter.sum();
    println!("{}", total);

    // 因此这里如果再次使用v1_iter会报错
    // println!("{:#?}", v1_iter);     // error[E0382]: borrow of moved value: `v1_iter`
}
