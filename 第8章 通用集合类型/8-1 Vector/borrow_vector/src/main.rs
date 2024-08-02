fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 此处是对v的一个不可变引用

    // 此处是对v的可变引用
    v.push(6); // error: cannot borrow `v` as mutable because it is also borrowed as immutable

    // 此处是对v的不可变引用
    println!("The first element is: {}", first);

    // 这里之所以对vector尾部做修改,而不允许引用vector头部的原因在于:
    // vector在内存中是连续存储的,如果在vector尾部添加元素,可能会导致vector重新分配内存,从而导致原来的引用失效
    // 而本例中的 first 仍然指向原来的内存地址,所以不允许对vector同时存在可变和不可变引用
}
