use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 这行代码实际上等价于: *((*value).borrow_mut()) += 10;
    // step1. 自动解引用
    // 由于 value 的类型为 Rc<T>,该类型实现了 Deref trait,因此Rust会自动解引用以便调用borrow_mut()方法
    // step2. 调用 borrow_mut() 方法
    // 该方法返回了一个 RefMut<T> 类型的智能指针,该指针实现了 DerefMut trait,因此可以通过 * 运算符来解引用
    // step3. 解引用
    // 智能指针RefMut<T>也实现了 DerefMut trait,因此可以通过 * 运算符来解引用,从而得到一个可变的内部值(这里是一个i32类型的值)
    *value.borrow_mut() += 10;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
