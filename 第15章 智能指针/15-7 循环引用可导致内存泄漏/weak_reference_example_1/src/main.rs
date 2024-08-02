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

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 此时 leaf的强引用计数值为1 弱引用计数值为0
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        // 子节点
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 此时 branch的强引用计数值为1 弱引用计数值为1
        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

        // 此时 leaf的强引用计数为2 弱引用计数值为0
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    } // 此处branch走出作用域 leaf的强引用减1 branch的强引用减1,此时branch的强引用计数为0,branch被释放

    // 此时 leaf指向的父节点已经被释放 所以此时leaf的父节点为空
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 此时 leaf的强引用计数为1 弱引用计数值为0
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
} // leaf走出作用域 leaf的强引用减1,此时leaf的强引用计数为0,leaf被释放
