#[derive(Debug)]
enum List<'a> {
    Cons(i32, Box<&'a List<'a>>),
    Nil,
}

fn main() {
    // 此处不能写成如下代码:
    // let a = List::Cons(5, Box::new(&List::Cons(10, Box::new(&List::Nil))));
    // 这是因为:
    // &List::Cons(10, Box::new(&List::Nil))是一个临时值,而临时值在表达式结束时就会被释放
    // 因此&List::Cons(10, Box::new(&List::Nil))的生命周期会比a短,这样就会出现悬垂引用
    // 为了解决这个问题,就需要将临时值绑定到一个变量上,使得这些临时值的生命周期延长到与 a b c这3个变量的生命周期相同

    // 这里不需要将&List::Nil绑定到变量上,因为 List::Nil 是一个静态值,它的生命周期是整个程序的生命周期
    // List::Nil 之所以是静态值,是因为它在编译时就确定的常量表达式
    let binding = List::Cons(10, Box::new(&List::Nil));
    let a = List::Cons(5, Box::new(&binding));

    let b = List::Cons(3, Box::new(&a));

    let c = List::Cons(4, Box::new(&a));

    println!("{:?}", b);
    println!("{:?}", c);
}
