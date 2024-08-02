use std::cell::RefCell;
use std::rc::Rc;

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
    let value = List::Cons(5, RefCell::new(Rc::new(List::Nil)));

    let a = List::Cons(10, RefCell::new(Rc::new(value)));

    println!("{:?}", a);

    if let Some(tail) = a.tail() {
        *tail.borrow_mut() = Rc::new(List::Cons(20, RefCell::new(Rc::new(List::Nil))));
    }

    println!("{:?}", a);
}
