use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // Rc<T> 类型只有当引用计数为0的时候才会被释放
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));

    // 此处调用Rc::clone()会使a的引用计数加1 也就是从1变成2
    // 因为Rc::new()的时候引用计数就是1 因为a本身要对这个数据进行引用 所以初态的 let a = Rc::new() 的时候引用计数就是1

    // 此处使用 a.clone() 和 Rc::clone()是等效的
    // 但是注意: 这个等效的前提是a的类型为 Rc<T>
    // 因为其他类型的clone()方法可能是深拷贝 而Rc<T>类型的 Rc<T>.clone()方法 与 Rc<T>::clone()关联函数是等效的

    let b = List::Cons(3, Rc::clone(&a));

    let c = List::Cons(4, Rc::clone(&a));

    println!("{:?}", b);
    println!("{:?}", c);
}
