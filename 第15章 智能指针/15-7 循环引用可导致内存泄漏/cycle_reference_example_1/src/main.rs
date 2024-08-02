use std::cell::RefCell;
use std::rc::Rc;
use crate::List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    /// tail 方法返回当前节点的下一个节点
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            // 此处的_表示忽略Cons中的第一个元素 只匹配第二个元素
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(List::Nil))));

    // a的强引用计数为1
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // a的下一个节点为Nil
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // 创建b后 a的强引用计数为2
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // b的强引用计数为1
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // b的下一个节点为a
    println!("b next item = {:?}", b.tail());

    // 修改a的下一个节点为b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // b的强引用计数为2
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // a的强引用计数为2
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 以下代码可以观察到循环引用 这会造成栈溢出
    println!("a next item = {:?}", a.tail());
}