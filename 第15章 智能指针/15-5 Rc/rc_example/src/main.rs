enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));

    let b = List::Cons(3, Box::new(a));

    // 此处a的所有权已经被转移给b, 因此不能再使用a
    let c = List::Cons(4, Box::new(a));  // error: value used after being moved
}
