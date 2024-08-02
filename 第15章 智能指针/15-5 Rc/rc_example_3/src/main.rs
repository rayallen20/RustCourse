use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    // 创建a之后 该数据的强引用计数为1
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = List::Cons(3, Rc::clone(&a));
    // 创建b之后 该数据的强引用计数+1(也就是2) 因为b和a都指向了同一个数据
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = List::Cons(4, Rc::clone(&a));
        // 创建c之后 该数据的强引用计数+1(也就是3)
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // c离开作用域之后 该数据的强引用计数-1(也就是2) 因为c离开作用域 已经被释放了

    // c离开作用域之后 该数据的强引用计数为2
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
} // Rc<T>也实现了Drop Trait,因此当Rc<T>类型离开作用域时,它的引用计数会减1;当引用计数为0时,它会调用Drop Trait来释放内存
