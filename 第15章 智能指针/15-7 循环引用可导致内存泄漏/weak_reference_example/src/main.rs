use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 叶子节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Weak<T>.upgrade()方法将返回一个Option<Rc<T>>类型
    // 若Weak<T>指向的Rc<T>对象未被释放,则返回 Some(Rc<T>)
    // 若Weak<T>指向的Rc<T>对象已被释放,则返回 None
    // 此时leaf.parent.borrow().upgrade()返回None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 子节点
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 为叶子节点添加指向父节点的弱引用
    // step1. leaf.parent.borrow_mut() 返回一个 RefMut<Weak<Node>> 类型,该类型也是一个智能指针
    // step2. 对该指针解引用并赋值为 Rc::downgrade(&branch) 注意Rc::downgrade(&branch)返回一个Weak<Node>类型
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 再次访问leaf.parent.borrow().upgrade()时,返回Some(Rc<Node>)
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
